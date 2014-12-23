#![feature(plugin_registrar, quote)]
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacItems, MacResult};
use syntax::parse::tts_to_parser;

mod block;
mod parser;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

fn expand_speculate(cx: &mut ExtCtxt, _sp: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
    let module = quote_item!(cx,
        mod sup {
        }
    ).expect("failed to create item!");

    let mut parser = tts_to_parser(cx.parse_sess(), tokens.to_vec(), cx.cfg());
    let parsed = parser::parse(&mut parser);
    println!("parsed = {}", parsed);

    MacItems::new(Some(module).into_iter())
}
