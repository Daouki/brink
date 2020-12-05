use crate::source_file::SourceSpan;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Item>,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<Item>,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ItemKind {
    LetBinding(LetBinding),
    Expr(Expr),
}

#[derive(Debug)]
pub struct LetBinding {
    pub identifier: Literal,
    pub body: LetBody,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum LetBody {
    Block(Block),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum LiteralKind {
    Identifier,
    Integer,
}
