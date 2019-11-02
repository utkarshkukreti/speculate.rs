use crate::block::{Bench, Block, Describe, It};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};

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
        let mut tests = Vec::new();

        if self.test_cases.is_some() && self.variables.is_some() {
            let mut parametrized = self.generate_parametrized(up);
            tests.append(&mut parametrized);
        } else {
            let test = self.generate_test(up);
            tests.push(test);
        }

        quote!(
            #(#tests)*
        )
    }
}

impl It {
    fn generate_parametrized(self, up: Option<&Describe>) -> Vec<TokenStream> {
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

        let mut tests = Vec::new();
        let attributes = self.attributes;

        let test_cases = self.test_cases.unwrap();
        let variables = self.variables.unwrap();

        for (case_name, param_values) in test_cases {
            let attributes_copy = attributes.clone();
            let stmts = flatten_blocks(blocks.clone());
            let name = Ident::new(
                &format!("test_{}_{}", self.name, case_name),
                self.name.span(),
            );

            let mut vars = Vec::with_capacity(variables.len());

            for (idx, expr) in param_values.iter().enumerate() {
                let var = variables.get(idx).unwrap();
                vars.push(quote!(
                    let #var = #expr;
                ));
            }

            tests.push(quote_spanned!(name.span() =>
                #[test]
                #(#attributes_copy)*
                fn #name() {
                    #(#vars)*
                    #(#stmts)*
                }
            ));
        }

        tests
    }

    fn generate_test(self, up: Option<&Describe>) -> TokenStream {
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

        let attributes = self.attributes;

        let stmts = flatten_blocks(blocks);

        let name = Ident::new(&format!("test_{}", self.name), self.name.span());

        quote_spanned!(name.span() =>
            #[test]
            #(#attributes)*
            fn #name() {
                #(#stmts)*
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
