#![feature(rustc_private)]
#![feature(plugin_registrar, quote)]
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::parse::stream_to_parser;
use syntax::tokenstream::{TokenStreamBuilder, TokenTree};
use syntax::util::small_vector::SmallVector;

use generator::Generate;

mod block;
mod generator;
mod parser;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

#[allow(unused_imports)]
fn expand_speculate(cx: &mut ExtCtxt, sp: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
    let mod_name = format!("speculate #{}", sp.lo().0);
    
    let mut parser = stream_to_parser(cx.parse_sess(), tokens.iter().cloned().collect());

    let block = parser::parse(&mod_name, &mut parser);
    let item = block.generate(cx, None);
    
    let module = item.map(|mut item| {
        item.attrs.push(quote_attr!(cx, #[allow(non_snake_case)]));
        
        if item.tokens.is_some() {
            let import = quote_tokens!(cx, #[allow(unused_imports)] use super::*;);
            let mut builder = TokenStreamBuilder::new();

            for tt in import {
                builder.push(tt);
            }

            item.tokens = Some(builder.add(item.tokens.unwrap()).build());
        }
        
        item
    });

    MacEager::items(SmallVector::one(module))
}
