use syntax::parse::common::seq_sep_none;
use syntax::parse::parser::Parser;
use syntax::parse::token;

use block::Block;

pub fn parse(parser: &mut Parser) -> Vec<Block> {
    return parser.parse_seq_to_end(&token::Eof, seq_sep_none(), recur);

    fn recur(parser: &mut Parser) -> Block {
        let span = parser.span;
        let ident = parser.parse_ident();

        match ident.as_str() {
            "it" => {
                let (name, _) = parser.parse_str();
                let block = parser.parse_block();

                Block::It {
                    name: name.get().to_string(),
                    block: block
                }
            },
            otherwise => {
                let message = format!("Expected `it`, found `{}`", otherwise);
                parser.span_fatal(span, message.as_slice())
            }
        }
    }
}
