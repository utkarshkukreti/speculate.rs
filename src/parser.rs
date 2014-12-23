use syntax::parse::parser::Parser;
use syntax::parse::token;

use block::Block;

pub fn parse(parser: &mut Parser) -> Block {
    parse_describe("sup", parser)
}

pub fn parse_describe(name: &str, parser: &mut Parser) -> Block {
    let brace = token::CloseDelim(token::Brace);

    let mut before = None;
    let mut after = None;
    let mut blocks = vec![];

    loop {
        if parser.token == brace {
            break
        }

        let span = parser.span;
        let ident = parser.parse_ident();

        match ident.as_str() {
            "describe" => {
                let (name, _) = parser.parse_str();
                parser.expect(&token::OpenDelim(token::Brace));
                blocks.push(parse_describe(name.get(), parser))
            },

            "it" => {
                let (name, _) = parser.parse_str();
                let block = parser.parse_block();

                blocks.push(Block::It {
                    name: name.get().to_string(),
                    block: block
                })
            },

            "before" => {
                before = Some(parser.parse_block());
            },

            "after" => {
                after = Some(parser.parse_block());
            },

            otherwise => {
                let message = format!("Expected `it`, found `{}`", otherwise);
                parser.span_fatal(span, message.as_slice())
            }
        }
    }

    Block::Describe {
        name: name.to_string(),
        before: before,
        after: after,
        blocks: blocks
    }
}
