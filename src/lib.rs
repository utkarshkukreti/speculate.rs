#![feature(rustc_private)]
#![feature(plugin_registrar, quote)]
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::parse::tts_to_parser;
use syntax::tokenstream::TokenTree;
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
    let mut parser = tts_to_parser(cx.parse_sess(), tokens.to_vec());
    let block = parser::parse(&mut parser);
    let item = block.generate(cx, None);

    let module = quote_item!(cx,
                             #[allow(non_snake_case)]
                             mod speculate {
                                 #[allow(unused_imports)]
                                 use super::*;
                                 $item
                             })
        .unwrap();

    MacEager::items(SmallVector::one(module))
}
