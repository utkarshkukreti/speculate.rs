use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::symbol::Symbol;

use block::{Block, Describe, It, Bench};

pub fn parse(parser: &mut Parser) -> Describe {
    parse_describe(Symbol::intern("_"), parser)
}

fn parse_describe(name: Symbol, parser: &mut Parser) -> Describe {
    let mut before = vec![];
    let mut after = vec![];
    let mut blocks = vec![];

    loop {
        if parser.token == token::CloseDelim(token::Brace) || parser.token == token::Eof {
            break;
        }

        let span = parser.span;
        if let token::Ident(ident) = parser.token {
            match &*ident.name.as_str() {
                "describe" | "context" => {
                    parser.bump();

                    let (name, _) = parser.parse_str().unwrap();
                    parser.expect(&token::OpenDelim(token::Brace)).unwrap();
                    let block = Block::Describe(parse_describe(name, parser));
                    parser.expect(&token::CloseDelim(token::Brace)).unwrap();
                    blocks.push(block);
                }

                "it" | "test" => {
                    parser.bump();

                    let (name, _) = parser.parse_str().unwrap();
                    let block = parser.parse_block().unwrap();

                    blocks.push(Block::It(It {
                        name: name.to_string(),
                        block: block,
                    }))
                }

                "bench" => {
                    parser.bump();

                    let (name, _) = parser.parse_str().unwrap();
                    parser.expect(&token::BinOp(token::Or)).unwrap();
                    let ident = parser.parse_ident().unwrap();
                    parser.expect(&token::BinOp(token::Or)).unwrap();
                    let block = parser.parse_block().unwrap();

                    blocks.push(Block::Bench(Bench {
                        name: name.to_string(),
                        ident: ident,
                        block: block,
                    }))
                }

                "before" => {
                    parser.bump();

                    before.push(parser.parse_block().unwrap());
                }

                "after" => {
                    parser.bump();

                    after.push(parser.parse_block().unwrap());
                }

                otherwise => {
                    if let Ok(item) = parser.parse_item() {
                        match item {
                            Some(item) => blocks.push(Block::Item(item)),
                            None => {}
                        }
                    } else {
                        let message = format!("Expected an item, `describe`, `context`, \
                                               `before`, `after`, `it`, `test`, or `bench`, \
                                               found `{}`",
                                              otherwise);
                        panic!("{:?}", parser.span_fatal(span, &message))
                    }
                }
            }
        } else {
            let message = format!("Expected an item, `describe`, `context`, \
                                   `before`, `after`, `it`, `test`, or `bench`, \
                                   found `{:?}`",
                                  parser.token);
            panic!("{:?}", parser.span_fatal(span, &message))
        }
    }

    Describe {
        name: name.to_string(),
        before: before,
        after: after,
        blocks: blocks,
    }
}
