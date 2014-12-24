use syntax::abi;
use syntax::ast;
use syntax::ast_util;
use syntax::codemap::DUMMY_SP;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::parse::token;

use block::Block;

pub fn generate(cx: &mut ExtCtxt, block: &Block) -> P<ast::Item> {
    match *block {
        Block::Describe {ref name, ref blocks, ..} => {
            let name = cx.ident_of(name.as_slice());
            let items = blocks.iter().map(|block| {
                generate(cx, block)
            }).collect::<Vec<_>>();

            cx.item_mod(DUMMY_SP,
                        DUMMY_SP,
                        name,
                        vec![],
                        vec![],
                        items)
        },

        Block::It {ref name, ref block} => {
            let name = cx.ident_of(name.as_slice());
            let attrs = vec![
                cx.attribute(
                    DUMMY_SP,
                    cx.meta_word(DUMMY_SP, token::InternedString::new("test"))
                )
            ];

            cx.item(DUMMY_SP,
                    name,
                    attrs,
                    ast::ItemFn(
                        cx.fn_decl(vec![], cx.ty(DUMMY_SP, ast::TyTup(vec![]))),
                        ast::Unsafety::Normal,
                        abi::Rust,
                        ast_util::empty_generics(),
                        block.clone()
                    ))
        }
    }
}
