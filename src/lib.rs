#![feature(plugin_registrar)]
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacExpr, MacResult};
use syntax::ext::build::AstBuilder;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("speculate", expand_speculate);
}

fn expand_speculate(cx: &mut ExtCtxt, sp: Span, _args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacExpr::new(cx.expr_uint(sp, 0))
}
