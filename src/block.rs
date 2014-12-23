use syntax::ast;
use syntax::ptr::P;

#[deriving(Show)]
pub enum Block {
    Describe {
        name: String,
        before: Option<P<ast::Block>>,
        after: Option<P<ast::Block>>,
        blocks: Vec<Block>
    },

    It {
        name: String,
        block: P<ast::Block>
    }
}
