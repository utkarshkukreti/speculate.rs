#![feature(rustc_private)]
#![feature(plugin_registrar, quote)]
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::attr::HasAttrs;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::parse::stream_to_parser;
use syntax::tokenstream::TokenTree;
use syntax::util::small_vector::SmallVector;

use generator::Generate;

mod block;
mod generator;
mod parser;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

fn expand_speculate(cx: &mut ExtCtxt, sp: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
    let mod_name = format!("speculate #{}", sp.lo().0);
    
    let mut parser = stream_to_parser(cx.parse_sess(), tokens.iter().cloned().collect());

    let block = parser::parse(&mod_name, &mut parser);
    let item = block.generate(cx, None).map_attrs(|mut attrs| {
        attrs.push(quote_attr!(cx, #[allow(non_snake_case, unused_imports)])); 
        attrs
    });

    MacEager::items(SmallVector::one(item))
}
