use syntax::ast;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;

use block::{Block, Describe, It, Bench};

pub fn parse(parser: &mut Parser) -> Describe {
    parse_describe("sup", parser)
}

fn parse_describe(name: &str, parser: &mut Parser) -> Describe {
    let brace = token::CloseDelim(token::Brace);

    let mut before = vec![];
    let mut after = vec![];
    let mut blocks = vec![];

    loop {
        if parser.token == brace || parser.token == token::Eof {
            break
        }

        let span = parser.span;
        let ident = parser.parse_ident().unwrap();

        match ident.as_str() {
            "describe" | "context" => {
                let (name, _) = parser.parse_str().unwrap();
                parser.expect(&token::OpenDelim(token::Brace)).unwrap();
                let block = Block::Describe(parse_describe(&name, parser));
                parser.expect(&token::CloseDelim(token::Brace)).unwrap();
                blocks.push(block);
            },

            "it" | "test" => {
                let (name, _) = parser.parse_str().unwrap();
                let block = parse_block(parser);

                blocks.push(Block::It(It {
                    name: name.to_string(),
                    block: block
                }))
            },

            "bench" => {
                let (name, _) = parser.parse_str().unwrap();
                parser.expect(&token::BinOp(token::Or)).unwrap();
                let ident = parser.parse_ident().unwrap();
                parser.expect(&token::BinOp(token::Or)).unwrap();
                let block = parse_block(parser);

                blocks.push(Block::Bench(Bench {
                    name: name.to_string(),
                    ident: ident,
                    block: block
                }))
            },

            "before" => {
                before.push(parse_block(parser));
            },

            "after" => {
                after.push(parse_block(parser));
            },

            otherwise => {
                let message = format!("Expected `describe`, `context`, \
`before`, `after`, `it`, `test`, or `bench`, found `{}`", otherwise);
                panic!("{}", parser.span_fatal(span, &message))
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
    let block = parser.parse_block().unwrap();
    if block.expr.is_some() {
        panic!("{}", parser.span_fatal(
            span,
            "last expression in this block must be terminated by `;`"))
    }
    block
}
