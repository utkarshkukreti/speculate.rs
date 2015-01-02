use syntax::ast;
use syntax::ptr::P;

#[deriving(Clone, Show)]
pub enum Block {
    Describe(Describe),
    It(It),
    Bench(Bench)
}

#[deriving(Clone, Show)]
pub struct Describe {
    pub name: String,
    pub before: Option<P<ast::Block>>,
    pub after: Option<P<ast::Block>>,
    pub blocks: Vec<Block>
}

#[deriving(Clone, Show)]
pub struct It {
    pub name: String,
    pub block: P<ast::Block>
}

#[deriving(Clone, Show)]
pub struct Bench {
    pub name: String,
    pub ident: ast::Ident,
    pub block: P<ast::Block>
}
