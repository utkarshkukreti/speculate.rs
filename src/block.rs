use syntax::ast;
use syntax::ptr::P;

#[deriving(Show)]
pub enum Block {
    Describe(Describe),
    It(It)
}

#[deriving(Show)]
pub struct Describe {
    pub name: String,
    pub before: Option<P<ast::Block>>,
    pub after: Option<P<ast::Block>>,
    pub blocks: Vec<Block>
}

#[deriving(Show)]
pub struct It {
    pub name: String,
    pub block: P<ast::Block>
}
