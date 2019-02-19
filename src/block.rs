use proc_macro2::Span;
use syn::{alt, braces, call, custom_keyword, do_parse, many0, named, punct, syn, synom::Synom};
use unicode_xid::UnicodeXID;

pub struct Root(pub(crate) Describe);

impl Synom for Root {
    named!(parse -> Self, do_parse!(
        mut content: many0!(syn!(DescribeBlock)) >>

        ({
            let mut before = vec![];
            let mut after  = vec![];
            let mut blocks = vec![];

            for block in content {
                match block {
                    DescribeBlock::Regular(block) => blocks.push(block),
                    DescribeBlock::Before(block)  => before.push(block),
                    DescribeBlock::After(block)   => after.push(block)
                }
            }

            Root(Describe {
                name: syn::Ident::new("speculate", Span::call_site()),
                before, after, blocks
            })
        })
    ));
}

#[derive(Clone)]
pub enum Block {
    Describe(Describe),
    It(It),
    Bench(Bench),
    Item(syn::Item),
}

impl Synom for Block {
    named!(parse -> Self, alt!(
        syn!(Describe)  => { Block::Describe }
        |
        syn!(It)        => { Block::It }
        |
        syn!(Bench)     => { Block::Bench }
        |
        syn!(syn::Item) => { Block::Item }
    ));
}

enum DescribeBlock {
    Regular(Block),
    Before(syn::Block),
    After(syn::Block),
}

impl Synom for DescribeBlock {
    named!(parse -> Self, alt!(
        do_parse!(
            custom_keyword!(before)             >>
            block: syn!(syn::Block)             >>
            (DescribeBlock::Before(block))      )
        |
        do_parse!(
            custom_keyword!(after)              >>
            block: syn!(syn::Block)             >>
            (DescribeBlock::After(block))       )
        |
        syn!(Block) => { DescribeBlock::Regular }
    ));
}

#[derive(Clone)]
pub struct Describe {
    pub name: syn::Ident,
    pub before: Vec<syn::Block>,
    pub after: Vec<syn::Block>,
    pub blocks: Vec<Block>,
}

impl Synom for Describe {
    named!(parse -> Self, do_parse!(
        alt!(custom_keyword!(describe) | custom_keyword!(context))  >>
        name: syn!(syn::LitStr)                                     >>
        root: braces!(syn!(Root))                                   >>

        ({
            let mut describe = (root.1).0;

            describe.name = litstr_to_ident(&name);
            describe
        })
    ));
}

#[derive(Clone)]
pub struct It {
    pub name: syn::Ident,
    pub attributes: Vec<syn::Attribute>,
    pub block: syn::Block,
}

impl Synom for It {
    named!(parse -> Self, do_parse!(
        attrs:   many0!(call!(syn::Attribute::parse_outer))         >>

        alt!(custom_keyword!(it) | custom_keyword!(test))           >>

        name:    syn!(syn::LitStr)                                  >>
        block:   syn!(syn::Block)                                   >>

        (It {
            name: litstr_to_ident(&name),
            attributes: attrs,
            block
        })
    ));
}

#[derive(Clone)]
pub struct Bench {
    pub name: syn::Ident,
    pub ident: syn::Ident,
    pub block: syn::Block,
}

impl Synom for Bench {
    named!(parse -> Self, do_parse!(
        alt!(custom_keyword!(bench) | custom_keyword!(test))        >>

        name:  syn!(syn::LitStr)                                    >>

        punct!(|)                                                   >>
        ident: syn!(syn::Ident)                                     >>
        punct!(|)                                                   >>

        block: syn!(syn::Block)                                     >>

        (Bench {
            name: litstr_to_ident(&name),
            ident, block
        })
    ));
}

fn litstr_to_ident(l: &syn::LitStr) -> syn::Ident {
    let string = l.value();
    let mut id = String::with_capacity(string.len());

    if string.is_empty() {
        return syn::Ident::new("_", l.span());
    }

    let mut chars = string.chars();
    let mut added_underscore = false;

    let first_ch = chars.next().unwrap();

    if !UnicodeXID::is_xid_start(first_ch) {
        id.push('_');

        if UnicodeXID::is_xid_continue(first_ch) {
            id.push(first_ch);
        } else {
            added_underscore = true;
        }
    } else {
        id.push(first_ch);
    }

    for ch in chars {
        if UnicodeXID::is_xid_continue(ch) {
            id.push(ch);
            added_underscore = false;
        } else if !added_underscore {
            id.push('_');
            added_underscore = true;
        }
    }

    if id.as_bytes()[id.len() - 1] == b'_' {
        id.pop();
    }

    syn::Ident::new(&id, l.span())
}
