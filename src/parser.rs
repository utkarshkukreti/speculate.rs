use syntax::ast;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;

use block::{Block, Describe, It, Bench};

pub fn parse(parser: &mut Parser) -> Describe {
    parse_describe("sup", parser)
}

pub fn parse_describe(name: &str, parser: &mut Parser) -> Describe {
    let brace = token::CloseDelim(token::Brace);

    let mut before = None;
    let mut after = None;
    let mut blocks = vec![];

    loop {
        if parser.token == brace || parser.token == token::Eof {
            break
        }

        let span = parser.span;
        let ident = parser.parse_ident();

        match ident.as_str() {
            "describe" | "context" => {
                let (name, _) = parser.parse_str();
                parser.expect(&token::OpenDelim(token::Brace));
                let block = Block::Describe(parse_describe(name.get(), parser));
                parser.expect(&token::CloseDelim(token::Brace));
                blocks.push(block);
            },

            "it" => {
                let (name, _) = parser.parse_str();
                let block = parse_block(parser);

                blocks.push(Block::It(It {
                    name: name.get().to_string(),
                    block: block
                }))
            },

            "bench" => {
                let (name, _) = parser.parse_str();
                parser.expect(&token::BinOp(token::Or));
                let ident = parser.parse_ident();
                parser.expect(&token::BinOp(token::Or));
                let block = parse_block(parser);

                blocks.push(Block::Bench(Bench {
                    name: name.get().to_string(),
                    ident: ident,
                    block: block
                }))
            },

            "before" => {
                before = Some(parse_block(parser));
            },

            "after" => {
                after = Some(parse_block(parser));
            },

            otherwise => {
                let message = format!("Expected \
`describe`, `context`, `before`, `after`, or `it`, found `{}`", otherwise);
                parser.span_fatal(span, &*message)
            }
        }
    }

    Describe {
        name: name.to_string(),
        before: before,
        after: after,
        blocks: blocks
    }
}

fn parse_block(parser: &mut Parser) -> P<ast::Block> {
    let span = parser.span;
    let block = parser.parse_block();
    if block.expr.is_some() {
        parser.span_fatal(
            span,
            "last expression in this block must be terminated by `;`")
    }
    block
}
