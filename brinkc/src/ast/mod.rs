use node_id::NodeId;

use crate::source_file::SourceSpan;

pub mod node_id;

#[derive(Debug)]
pub struct Program {
    pub id: NodeId,
    pub body: Vec<Item>,
}

#[derive(Debug)]
pub struct Block {
    pub id: NodeId,
    pub span: SourceSpan,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub id: NodeId,
    pub span: SourceSpan,
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    LetBinding(LetBinding),
    Expr(Expr),
}

#[derive(Debug)]
pub struct LetBinding {
    pub id: NodeId,
    pub span: SourceSpan,
    pub identifier: Literal,
    pub body: LetBody,
}

#[derive(Debug)]
pub enum LetBody {
    Block(Block),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Expr {
    pub id: NodeId,
    pub span: SourceSpan,
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug)]
pub struct Literal {
    pub id: NodeId,
    pub span: SourceSpan,
    pub kind: LiteralKind,
}

#[derive(Debug)]
pub enum LiteralKind {
    Identifier,
    Integer,
}
