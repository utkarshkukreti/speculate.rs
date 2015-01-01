#![feature(plugin_registrar, slicing_syntax)]
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::{mod, TokenTree};
use syntax::codemap::DUMMY_SP;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacItems, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::parse::tts_to_parser;

use generator::Generate;

mod block;
mod parser;
mod generator;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

fn expand_speculate(cx: &mut ExtCtxt, _sp: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = tts_to_parser(cx.parse_sess(), tokens.to_vec(), cx.cfg());
    let block = parser::parse(&mut parser);
    let item = block.generate(cx, None);

    let attrs = vec![
        cx.attribute(
            DUMMY_SP,
            cx.meta_list(
                DUMMY_SP,
                token::InternedString::new("allow"),
                vec![
                    cx.meta_word(
                        DUMMY_SP,
                        token::InternedString::new("non_snake_case"))
                ]
            )
        )
    ];

    let pub_use_super_star = cx.view_use_glob(DUMMY_SP,
                                              ast::Visibility::Public,
                                              vec![cx.ident_of("super")]);

    let module = cx.item_mod(DUMMY_SP, DUMMY_SP, cx.ident_of("sup"),
                             attrs, vec![pub_use_super_star], vec![item]);

    MacItems::new(Some(module).into_iter())
}
