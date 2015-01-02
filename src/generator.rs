use syntax::abi;
use syntax::ast;
use syntax::ast_util;
use syntax::codemap::DUMMY_SP;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::parse::token;

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
        let name = cx.ident_of(self.name[]);

        if let Some(ref up) = up {
            if let Some(ref before) = up.before {
                self.before = match self.before {
                    Some(ref now) => Some(merge_blocks(before, now)),
                    None => Some(before.clone())
                }
            }

            if let Some(ref after) = up.after {
                self.after = match self.after {
                    Some(ref now) => Some(merge_blocks(now, after)),
                    None => Some(after.clone())
                }
            }
        }

        let items = self.blocks.iter().map(|block| {
            block.clone().generate(cx, Some(&self))
        }).collect();

        let pub_use_super_star = cx.view_use_glob(DUMMY_SP,
                                                  ast::Visibility::Public,
                                                  vec![cx.ident_of("super")]);

        cx.item_mod(DUMMY_SP,
                    DUMMY_SP,
                    name,
                    vec![],
                    vec![pub_use_super_star],
                    items)
    }
}

impl Generate for It {
    fn generate(self, cx: &mut ExtCtxt, up: Option<&Describe>) -> P<ast::Item> {
        let name = cx.ident_of(self.name[]);
        let attrs = vec![
            cx.attribute(
                DUMMY_SP,
                cx.meta_word(DUMMY_SP, token::InternedString::new("test"))
            )
        ];

        let block = if let Some(ref up) = up {
            match (&up.before, &up.after) {
                (&Some(ref before), &Some(ref after)) => {
                    merge_blocks(&merge_blocks(before, &self.block), after)
                },
                (&Some(ref before), &None) => merge_blocks(before, &self.block),
                (&None, &Some(ref after)) => merge_blocks(&self.block, after),
                (&None, &None) => self.block.clone()
            }
        } else {
            self.block
        };

        cx.item(DUMMY_SP, name, attrs,
                ast::ItemFn(
                    cx.fn_decl(vec![], cx.ty(DUMMY_SP, ast::TyTup(vec![]))),
                    ast::Unsafety::Normal,
                    abi::Rust,
                    ast_util::empty_generics(),
                    block
                ))
    }
}

impl Generate for Bench {
    fn generate(self, cx: &mut ExtCtxt, _up: Option<&Describe>) -> P<ast::Item> {
        let name = cx.ident_of(self.name[]);

        let attrs = vec![
            cx.attribute(
                DUMMY_SP,
                cx.meta_word(DUMMY_SP, token::InternedString::new("bench"))
            )
        ];

        let args = vec![
            cx.arg(DUMMY_SP, self.ident, quote_ty!(cx, &mut ::test::Bencher))
        ];

        cx.item(DUMMY_SP, name, attrs,
                ast::ItemFn(
                    cx.fn_decl(args, cx.ty(DUMMY_SP, ast::TyTup(vec![]))),
                    ast::Unsafety::Normal,
                    abi::Rust,
                    ast_util::empty_generics(),
                    self.block
                ))
    }
}

fn merge_blocks(left: &P<ast::Block>, right: &P<ast::Block>) -> P<ast::Block> {
    P(ast::Block {
        view_items: left.view_items.clone() + right.view_items[],
        stmts: left.stmts.clone() + right.stmts[],
        ..left.deref().clone()
    })
}
