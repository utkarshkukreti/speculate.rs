#![feature(proc_macro, proc_macro_span, rustc_private)]
extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod block;
mod generator;

use block::Root;
use generator::Generate;

use proc_macro::TokenStream;

fn transform(name: &str, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let mut root = syn::parse2::<Root>(input).unwrap();

    root.0.name = syn::Ident::new(name, proc_macro2::Span::call_site());

    let mut prefix = quote!( #[allow(non_snake_case)] );
    let modl = root.0.generate(None);

    prefix.extend(modl);
    prefix.into()
}

#[proc_macro]
pub fn speculate(input: TokenStream) -> TokenStream {
    // NOTE: We cannot use the name 'speculate' for the generated module, as it conflicts
    // with the imported symbol 'speculate.'
    transform("speculate_tests", input)
}

#[proc_macro]
pub fn speculate_again(input: TokenStream) -> TokenStream {
    let start = proc_macro::Span::call_site().start().line;

    transform(&format!("speculate_tests_line_{}", start), input)
}
