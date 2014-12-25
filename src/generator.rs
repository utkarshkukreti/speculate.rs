use syntax::abi;
use syntax::ast;
use syntax::ast_util;
use syntax::codemap::DUMMY_SP;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::parse::token;

use block::{Block, Describe, It};

pub trait Generate {
    fn generate(self, cx: &mut ExtCtxt) -> P<ast::Item>;
}

impl Generate for Block {
    fn generate(self, cx: &mut ExtCtxt) -> P<ast::Item> {
        match self {
            Block::Describe(describe) => describe.generate(cx),
            Block::It(it) => it.generate(cx)
        }
    }
}

impl Generate for Describe {
    fn generate(self, cx: &mut ExtCtxt) -> P<ast::Item> {
        let name = cx.ident_of(self.name.as_slice());
        let items = self.blocks.into_iter().map(|block| {
            block.generate(cx)
        }).collect();

        cx.item_mod(DUMMY_SP, DUMMY_SP, name, vec![], vec![], items)
    }
}

impl Generate for It {
    fn generate(self, cx: &mut ExtCtxt) -> P<ast::Item> {
        let name = cx.ident_of(self.name.as_slice());
        let attrs = vec![
            cx.attribute(
                DUMMY_SP,
                cx.meta_word(DUMMY_SP, token::InternedString::new("test"))
            )
        ];

        cx.item(DUMMY_SP, name, attrs,
                ast::ItemFn(
                    cx.fn_decl(vec![], cx.ty(DUMMY_SP, ast::TyTup(vec![]))),
                    ast::Unsafety::Normal,
                    abi::Rust,
                    ast_util::empty_generics(),
                    self.block.clone()
                ))
    }
}
