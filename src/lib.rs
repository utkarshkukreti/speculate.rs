#![feature(plugin_registrar, quote)]
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacItems, MacResult};

mod block;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

fn expand_speculate(cx: &mut ExtCtxt, _sp: Span, _args: &[TokenTree]) -> Box<MacResult + 'static> {
    let module = quote_item!(cx,
        mod sup {
        }
    ).expect("failed to create item!");

    MacItems::new(Some(module).into_iter())
}
