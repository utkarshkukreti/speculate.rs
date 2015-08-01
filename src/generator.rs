use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

use block::{Block, Describe, It, Bench};

pub trait Generate {
    fn generate(self, cx: &mut ExtCtxt, up: Option<&Describe>) -> P<ast::Item>;
}

impl Generate for Block {
    fn generate(self, cx: &mut ExtCtxt, up: Option<&Describe>) -> P<ast::Item> {
        match self {
            Block::Describe(describe) => describe.generate(cx, up),
            Block::It(it) => it.generate(cx, up),
            Block::Bench(bench) => bench.generate(cx, up)
        }
    }
}

impl Generate for Describe {
    fn generate(mut self,
                cx: &mut ExtCtxt,
                up: Option<&Describe>) -> P<ast::Item> {
        let name = cx.ident_of(&self.name);

        if let Some(ref up) = up {
            self.before = up.before.iter()
                .chain(self.before.iter())
                .cloned()
                .collect();
            self.after = self.after.iter()
                .chain(up.after.iter())
                .cloned()
                .collect();
        }

        let items = self.blocks.iter().map(|block| {
            block.clone().generate(cx, Some(&self))
        }).collect::<Vec<_>>();

        quote_item!(cx, mod $name {
            pub use super::*;
            $items
        }).unwrap()
    }
}

impl Generate for It {
    fn generate(self, cx: &mut ExtCtxt, up: Option<&Describe>) -> P<ast::Item> {
        let name = cx.ident_of(&self.name);

        let block = if let Some(ref up) = up {
            up.before.iter()
                .chain(Some(self.block).iter())
                .chain(up.after.iter())
                .cloned().collect()
        } else {
            vec![self.block]
        };

        let mut block = block.into_iter();
        let head = block.next().unwrap();
        let block = block.fold(head, merge_blocks);

        quote_item!(cx, #[test] fn $name() { $block }).unwrap()
    }
}

impl Generate for Bench {
    fn generate(self, cx: &mut ExtCtxt, up: Option<&Describe>) -> P<ast::Item> {
        let name = cx.ident_of(&self.name);

        let block = if let Some(ref up) = up {
            up.before.iter()
                .chain(Some(self.block).iter())
                .chain(up.after.iter())
                .cloned().collect()
        } else {
            vec![self.block]
        };

        let mut block = block.into_iter();
        let head = block.next().unwrap();
        let block = block.fold(head, merge_blocks);

        let ident = self.ident;
        quote_item!(cx, #[bench] fn $name($ident: &mut ::test::Bencher) {
            $block
        }).unwrap()
    }
}

fn merge_blocks(left: P<ast::Block>, right: P<ast::Block>) -> P<ast::Block> {
    P(ast::Block {
        stmts: left.stmts.iter().chain(right.stmts.iter()).cloned().collect(),
        ..(*left).clone()
    })
}
