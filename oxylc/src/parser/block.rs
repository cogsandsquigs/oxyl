use super::errors::ParserError;
use crate::{
    ast::block::Block,
    parser::{expression::expression, statement::statement},
};
use errgonomic::{
    combinators::{commit, is, many, maybe, whitespace},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a block expression.
/// ```bnf
/// <block> ::= "{" <statement>* <expression> "}"
/// ```
pub fn block(state: State<&str, ParserError>) -> Result<&str, Block, ParserError> {
    is("{")
        .then(commit(
            many(statement)
                .then(expression)
                .then(maybe(whitespace))
                .then(is("}")),
        ))
        .map_with_state(|state, (open_curly, (((statements, expression), _), _))| {
            let location = state.as_input().span().union_between(open_curly.span());
            (
                state,
                Block::new(statements, Box::new(expression), location),
            )
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        expression::{Expression, ExpressionKind},
        identifier::Identifier,
        statement::{Statement, StatementKind},
        value::{Value, ValueKind},
        AstNode,
    };
    use errgonomic::parser::input::Span;

    #[test]
    fn can_parse_only_expr_block() {
        let (state, expr) = block.process("{ 123 }".into()).unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(
            expr,
            Block::new(
                vec![],
                Box::new(Expression::new(
                    ExpressionKind::Value(Value::new(ValueKind::Integer(123), (2..5).into())),
                    (2..5).into()
                )),
                (0..7).into()
            )
        );
    }

    #[test]
    fn can_parse_stmts_in_block() {
        let (state, expr) = block
            .process(
                "{ let x = 3 
            \t\r\n123\n\n}"
                    .into(),
            )
            .unwrap();
        assert!(state.is_ok());
        assert_eq!(state.as_input().as_inner(), "");
        assert_eq!(expr.location(), &Span::from(0..34));
        assert_eq!(expr.statements().len(), 1);
        assert_eq!(
            expr.statements(),
            &[Statement::new(
                StatementKind::Let {
                    is_mutable: false,
                    ident: Identifier::new("x".into(), (6..7).into()),
                    expression: Expression::new(
                        ExpressionKind::Value(Value::new(ValueKind::Integer(3), (10..11).into())),
                        (10..11).into()
                    ),
                },
                (2..13).into()
            )]
        );
        assert_eq!(
            expr.expression(),
            &Expression::new(
                ExpressionKind::Value(Value::new(ValueKind::Integer(123), (28..31).into())),
                (28..31).into()
            )
        );
    }
}
