use syntax::ast;
use syntax::ptr::P;

#[derive(Clone, Show)]
pub enum Block {
    Describe(Describe),
    It(It),
    Bench(Bench)
}

#[derive(Clone, Show)]
pub struct Describe {
    pub name: String,
    pub before: Option<P<ast::Block>>,
    pub after: Option<P<ast::Block>>,
    pub blocks: Vec<Block>
}

#[derive(Clone, Show)]
pub struct It {
    pub name: String,
    pub block: P<ast::Block>
}

#[derive(Clone, Show)]
pub struct Bench {
    pub name: String,
    pub ident: ast::Ident,
    pub block: P<ast::Block>
}
