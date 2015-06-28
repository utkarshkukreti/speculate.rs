#![feature(rustc_private)]
#![feature(plugin_registrar, quote)]
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::{self, TokenTree};
use syntax::codemap::DUMMY_SP;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::tts_to_parser;
use syntax::util::small_vector::SmallVector;

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

    let attrs = vec![quote_attr!(cx, #[allow(non_snake_case)])];

    let pub_use_super_star = cx.item_use_glob(DUMMY_SP,
                                              ast::Visibility::Public,
                                              vec![cx.ident_of("super")]);

    let module = cx.item_mod(DUMMY_SP, DUMMY_SP, cx.ident_of("sup"),
                             attrs, vec![pub_use_super_star, item]);

    MacEager::items(SmallVector::one(module))
}
