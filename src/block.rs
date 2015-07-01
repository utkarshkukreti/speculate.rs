use syntax::ast;
use syntax::ptr::P;

#[derive(Clone, Debug)]
pub enum Block {
    Describe(Describe),
    It(It),
    Bench(Bench)
}

#[derive(Clone, Debug)]
pub struct Describe {
    pub name: String,
    pub before: Vec<P<ast::Block>>,
    pub after: Vec<P<ast::Block>>,
    pub blocks: Vec<Block>
}

#[derive(Clone, Debug)]
pub struct It {
    pub name: String,
    pub block: P<ast::Block>
}

#[derive(Clone, Debug)]
pub struct Bench {
    pub name: String,
    pub ident: ast::Ident,
    pub block: P<ast::Block>
}
