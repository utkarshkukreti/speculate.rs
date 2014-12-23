use syntax::ast;
use syntax::ptr::P;

pub enum Block {
    Describe {
        name: String,
        blocks: Vec<Block>
    },

    It {
        name: String,
        block: P<ast::Block>
    }
}
