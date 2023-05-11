use crate::block::{Bench, Block, Describe, It};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Stmt;

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
            self.before = up.before.iter().chain(self.before.iter()).cloned().collect();
            self.after = self.after.iter().chain(up.after.iter()).cloned().collect();
        }

        let name = &self.name;
        let blocks: Vec<TokenStream> = self.blocks.iter().map(|block| block.clone().generate(Some(&self))).collect();

        quote! {
            mod #name {
                #[allow(unused_imports)]
                use super::*;

                #(#blocks)*
            }
        }
    }
}

impl Generate for It {
    fn generate(self, up: Option<&Describe>) -> TokenStream {
        let stmts: Vec<Stmt> = if let Some(ref up) = up {
            let mut combined = up.before.clone();
            combined.extend(self.block.stmts.iter().cloned());
            combined.extend(up.after.iter().cloned());
            combined
        } else {
            self.block.stmts.clone()
        };
        let stmts_token: Vec<TokenStream> = stmts.into_iter().map(|stmt| quote! { #stmt }).collect();

        let name = &self.name;
        let attributes = &self.attributes;

        quote! {
            #(#attributes)*
            #[test]
            fn #name() {
                #(#stmts_token)*
            }
        }
    }
}

impl Generate for Bench {
    fn generate(self, up: Option<&Describe>) -> TokenStream {
        let stmts: Vec<Stmt> = if let Some(ref up) = up {
            let mut combined = up.before.clone();
            combined.extend(self.block.stmts.iter().cloned());
            combined.extend(up.after.iter().cloned());
            combined
        } else {
            self.block.stmts.clone()
        };
        let stmts_token: Vec<TokenStream> = stmts.into_iter().map(|stmt| quote! { #stmt }).collect();

        let name = &self.name;
        let ident = &self.ident;

        quote! {
            #[bench]
            fn #name(#ident: &mut ::test::Bencher) {
                #(#stmts_token)*
            }
        }
    }
}
