use crate::block::{Bench, Block, Describe, It};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

pub trait Generate {
    fn generate(self, up: Option<&Describe>) -> TokenStream;
}

impl Generate for Block {
    fn generate(self, up: Option<&Describe>) -> TokenStream {
        match self {
            Block::Describe(describe) => describe.generate(up),
            Block::It(it) => it.generate(up),
            Block::Bench(bench) => bench.generate(up),
            Block::Item(item) => item.into_token_stream(),
        }
    }
}

impl Generate for Describe {
    fn generate(mut self, up: Option<&Describe>) -> TokenStream {
        if let Some(ref up) = up {
            self.before = up
                .before
                .iter()
                .chain(self.before.iter())
                .cloned()
                .collect();
            self.after = self.after.iter().chain(up.after.iter()).cloned().collect();
        }

        let items = self
            .blocks
            .iter()
            .map(|block| block.clone().generate(Some(&self)))
            .collect::<Vec<_>>();

        let name = &self.name;

        quote_spanned!(name.span() =>
            mod #name {
                #[allow(unused_imports)]
                use super::*;

                #(#items)*
            }
        )
    }
}

impl Generate for It {
    fn generate(self, up: Option<&Describe>) -> TokenStream {
        let blocks = if let Some(ref up) = up {
            up.before
                .iter()
                .chain(Some(self.block).iter())
                .chain(up.after.iter())
                .cloned()
                .collect()
        } else {
            vec![self.block]
        };

        let stmts = flatten_blocks(blocks);

        let name = Ident::new(&format!("test_{}", self.name), self.name.span());
        let attributes = self.attributes;
        let (ret_type, ret_val) = return_signature(up.and_then(|d| d.errtype.clone()), &attributes);

        quote_spanned!(name.span() =>
            #[test]
            #(#attributes)*
            fn #name() -> #ret_type {
                #(#stmts)*
                #ret_val
            }
        )
    }
}

impl Generate for Bench {
    fn generate(self, up: Option<&Describe>) -> TokenStream {
        let blocks = if let Some(ref up) = up {
            up.before
                .iter()
                .chain(Some(self.block).iter())
                .chain(up.after.iter())
                .cloned()
                .collect()
        } else {
            vec![self.block]
        };

        let stmts = flatten_blocks(blocks);

        let name = Ident::new(&format!("bench_{}", self.name), self.name.span());
        let ident = self.ident;

        quote_spanned!(name.span() =>
            #[bench]
            fn #name(#ident: &mut ::test::Bencher) {
                #(#stmts)*
            }
        )
    }
}

fn flatten_blocks(blocks: Vec<syn::Block>) -> impl Iterator<Item = syn::Stmt> {
    blocks.into_iter().flat_map(|block| block.stmts)
}

fn return_signature(err: Option<syn::TypePath>, attributes: &Vec<syn::Attribute>) -> (TokenStream, TokenStream) {
    let should_panic = attributes.iter()
        .find(|attr| attr.path.segments.first()
            .filter(|segment| segment.value().ident == "should_panic")
            .is_some());

    match (err, should_panic) {
        (Some(ref errtype), None) => (
            quote_spanned!(errtype.span()=> Result<(), #errtype>),
            quote_spanned!(errtype.span()=> Ok(()))
        ),
        _ => (
            quote!{ () },
            quote!{ }
        ),
    }
}
