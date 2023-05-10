use syn::{ custom_keyword,
           parse::{ Parse, ParseStream },
           Error, LitStr, Stmt };
use unicode_ident::{is_xid_continue, is_xid_start};

#[cfg(not(feature = "nightly"))]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "nightly")]
fn get_root_name() -> proc_macro2::Ident {
    let start_line = proc_macro::Span::call_site().start().line;
    let module_name = format!("speculate_{}", start_line);

    syn::Ident::new(&module_name, proc_macro2::Span::call_site())
}

// TODO: Get rid of this once proc_macro_span stabilises
#[cfg(not(feature = "nightly"))]
static GLOBAL_SPECULATE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(feature = "nightly"))]
fn get_root_name() -> proc_macro2::Ident {
    let count = GLOBAL_SPECULATE_COUNT.fetch_add(1, Ordering::SeqCst);
    let module_name = format!("speculate_{}", count);

    syn::Ident::new(&module_name, proc_macro2::Span::call_site())
}

custom_keyword!(describe);
custom_keyword!(context);
custom_keyword!(it);
custom_keyword!(test);
custom_keyword!(before);
custom_keyword!(after);
custom_keyword!(bench);

#[derive(Clone)]
pub enum Block {
    Describe(Describe),
    It(It),
    Bench(Bench),
    Item(syn::Item),
}

fn parse_block(input: ParseStream) -> Result<Option<Block>, Error> {
    if input.is_empty() {
        return Ok(None);
    }

    let forked_input = input.fork();
    let lookahead = input.lookahead1();

    if lookahead.peek(syn::token::Pound) {
        // If the next token is '#', parse as an `It` block.
        let block = input.parse::<It>()?;
        Ok(Some(Block::It(block)))
    } else if lookahead.peek(describe) {
        let _: describe = input.parse()?;
        Ok(Some(input.parse::<Describe>().map(Block::Describe)?))
    } else if lookahead.peek(context) {
        let _: context = input.parse()?;
        Ok(Some(input.parse::<Describe>().map(Block::Describe)?))
    } else if lookahead.peek(it) || lookahead.peek(test) {
        Ok(Some(input.parse::<It>().map(Block::It)?))
    } else if lookahead.peek(bench) {
        Ok(Some(input.parse::<Bench>().map(Block::Bench)?))
    } else if let Ok(item) = forked_input.parse::<syn::Item>() {
        input.parse::<syn::Item>()?;
        Ok(Some(Block::Item(item)))
    } else {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct Describe {
    pub name: syn::Ident,
    pub before: Vec<Stmt>,
    pub after: Vec<Stmt>,
    pub blocks: Vec<Block>,
}

fn parse_content(input: ParseStream) -> Result<(Vec<Stmt>, Vec<Stmt>, Vec<Block>), Error> {
    let mut before_vec: Vec<Stmt> = Vec::new();
    let mut after_vec: Vec<Stmt> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();

    while !input.is_empty() {
        if let Some(block) = parse_block(&input)? {
            blocks.push(block);
        } else {
            let lookahead = input.lookahead1();

            if lookahead.peek(before) {
                let _: before = input.parse()?;
                let before_block = input.parse::<syn::Block>()?;
                for stmt in before_block.stmts {
                    before_vec.push(stmt);
                }
            } else if lookahead.peek(after) {
                let _: after = input.parse()?;
                let after_block = input.parse::<syn::Block>()?;
                for stmt in after_block.stmts {
                    after_vec.push(stmt);
                }
            } else {
                return Err(Error::new(input.span(), "Expected a block or 'before' or 'after'."));
            }
        }
    }
    blocks.reverse();
    Ok((before_vec, after_vec, blocks))
}

impl Parse for Describe {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        let name: syn::Ident = if lookahead.peek(syn::LitStr) {
            let name: syn::LitStr = input.parse()?;
            litstr_to_ident(&name)
        } else {
            get_root_name()
        };

        let before_vec: Vec<Stmt>;
        let after_vec: Vec<Stmt>;
        let blocks: Vec<Block>;

        if input.peek(syn::token::Brace) {
            let content_inside_braces;
            let _brace_token = syn::braced!(content_inside_braces in input);
            (before_vec, after_vec, blocks) = parse_content(&content_inside_braces)?;
        } else {
            (before_vec, after_vec, blocks) = parse_content(input)?;
        }
        let describe = Describe {
            name,
            before: before_vec,
            after: after_vec,
            blocks,
        };
        Ok(describe)
    }
}

#[derive(Clone)]
pub struct It {
    pub name: syn::Ident,
    pub attributes: Vec<syn::Attribute>,
    pub block: syn::Block,
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attributes: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(it) {
            let _: it = input.parse()?;
        } else if lookahead.peek(test) {
            let _: test = input.parse()?;
        } else {
            return Err(lookahead.error());
        }

        let name: LitStr = input.parse()?;
        let block = input.parse::<syn::Block>()?;

        Ok(It {
            name: litstr_to_ident(&name),
            attributes,
            block,
        })
    }
}

#[derive(Clone)]
pub struct Bench {
    pub name: syn::Ident,
    pub ident: syn::Ident,
    pub block: syn::Block,
}

impl Parse for Bench {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let _: bench = input.parse()?;
        let name: LitStr = input.parse()?;
        let ident = input.parse()?;
        let block = input.parse()?;

        Ok(Bench { name: litstr_to_ident(&name), ident, block })
    }
}

fn litstr_to_ident(l: &LitStr) -> syn::Ident {
    let string = l.value();

    if string.is_empty() {
        return syn::Ident::new("_", l.span());
    }

    let mut id: String = string
        .chars()
        .enumerate()
        .flat_map(|(i, ch)| {
            if i == 0 && !is_xid_start(ch) {
                if is_xid_continue(ch) {
                    vec!['_', ch]
                } else {
                    vec!['_']
                }
            } else if is_xid_continue(ch) {
                vec![ch]
            } else {
                vec!['_']
            }
        })
        .collect();

    while id.contains("__") {
        id = id.replace("__", "_");
    }

    id = id.trim_end_matches('_').to_string();

    syn::Ident::new(&id, l.span())
}
