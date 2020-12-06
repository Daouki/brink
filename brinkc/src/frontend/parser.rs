use crate::{
    ast::{self, node_id::NodeIdGenerator},
    source_file::SourceSpan,
};

use super::{
    parse_session::ParseSession,
    token::{Token, TokenKind},
    tokens::Tokens,
};

#[allow(dead_code)]
pub struct Parser<'a> {
    session: &'a ParseSession,
    tokens: Tokens,
    id_generator: NodeIdGenerator,
}

impl<'a> Parser<'a> {
    pub fn parse(session: &'a ParseSession, tokens: Tokens) -> ast::Program {
        let mut parser = Self {
            session,
            tokens,
            id_generator: NodeIdGenerator::new(),
        };
        parser.parse_program()
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut body = Vec::new();
        while !self.tokens.at_end() {
            body.push(self.parse_item().unwrap());
        }
        ast::Program {
            id: self.id_generator.next_id(),
            body,
        }
    }

    fn parse_block(&mut self) -> Result<ast::Block, ParseError> {
        let start = self.expect(TokenKind::Indent)?.span.start;
        let mut items = Vec::new();
        while !self.tokens.at_end() && !self.tokens.check(TokenKind::Dedent) {
            items.push(self.parse_item()?);
        }
        let _ = self.tokens.consume(TokenKind::Dedent);
        let span = SourceSpan::new(start, self.tokens.previous().span.end);
        Ok(ast::Block {
            id: self.id_generator.next_id(),
            span,
            items,
        })
    }

    fn parse_item(&mut self) -> Result<ast::Item, ParseError> {
        if self.tokens.consume(TokenKind::Let).is_some() {
            let let_binding = self.parse_let_binding()?;
            let span = let_binding.span.clone();
            Ok(ast::Item {
                id: self.id_generator.next_id(),
                span,
                kind: ast::ItemKind::LetBinding(let_binding),
            })
        } else {
            let expr = self.parse_expr()?;
            let span = expr.span.clone();
            Ok(ast::Item {
                id: self.id_generator.next_id(),
                span,
                kind: ast::ItemKind::Expr(expr),
            })
        }
    }

    fn parse_let_binding(&mut self) -> Result<ast::LetBinding, ParseError> {
        let start = self.tokens.previous().span.start;
        let identifier = self.expect_identifier()?;
        let _ = self.expect(TokenKind::Equal)?;
        let body = self.parse_let_binding_body()?;
        let span = SourceSpan::new(start, self.tokens.previous().span.end);
        Ok(ast::LetBinding {
            id: self.id_generator.next_id(),
            span,
            identifier,
            body,
        })
    }

    fn parse_let_binding_body(&mut self) -> Result<ast::LetBody, ParseError> {
        if self.tokens.check(TokenKind::Indent) {
            Ok(ast::LetBody::Block(self.parse_block()?))
        } else {
            let expr = self.parse_expr()?;
            let _ = self.expect(TokenKind::NewLine)?;
            Ok(ast::LetBody::Expr(expr))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expr, ParseError> {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<ast::Expr, ParseError> {
        if self.tokens.check(TokenKind::Integer) {
            let token = self.tokens.consume(TokenKind::Integer).unwrap();
            Ok(ast::Expr {
                id: self.id_generator.next_id(),
                span: token.span,
                kind: ast::ExprKind::Literal(ast::Literal {
                    id: self.id_generator.next_id(),
                    span: token.span,
                    kind: ast::LiteralKind::Integer,
                }),
            })
        } else {
            let current = self.tokens.peek();
            Err(ParseError {
                span: current.span,
                message: format!("expected an expression, but found {:?}", self.tokens.peek()),
            })
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if let Some(token) = self.tokens.consume(kind) {
            Ok(token)
        } else {
            let current = self.tokens.peek();
            Err(ParseError {
                span: current.span,
                message: format!(r#"expected "{:?}", but found "{:?}""#, kind, current.kind),
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<ast::Literal, ParseError> {
        if let Some(token) = self.tokens.consume(TokenKind::Identifier) {
            Ok(ast::Literal {
                id: self.id_generator.next_id(),
                span: token.span,
                kind: ast::LiteralKind::Identifier,
            })
        } else {
            let current = self.tokens.peek();
            Err(ParseError {
                span: current.span,
                message: format!(r#"expected {{identifier}}, but found "{:?}""#, current.kind),
            })
        }
    }
}

#[derive(Debug)]
struct ParseError {
    pub span: SourceSpan,
    pub message: String,
}
