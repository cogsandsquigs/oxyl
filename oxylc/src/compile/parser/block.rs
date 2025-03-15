use super::{errors::ParserError, expression::expression, statement::statement, utils::ww};
use crate::fst::block::Block;
use errgonomic::{
    combinators::{commit, is, many},
    parser::{errors::Result, state::State, Parser},
};

/// Parses a block expression.
/// ```bnf
/// <block> ::= "{" <statement>* <expression> "}"
/// ```
pub fn block(state: State<&str, ParserError>) -> Result<&str, Block, ParserError> {
    is("{")
        .then(commit(many(statement).then(ww(expression)).then(is("}"))))
        .map_with_state(|state, (open_curly, ((statements, expression), _))| {
            let location = state.as_input().span().union_between(open_curly.span());
            (
                state,
                Block::new(location, statements, Box::new(expression)),
            )
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fst::{
        expression::{Expression, ExpressionKind},
        identifier::Identifier,
        statement::{Statement, StatementKind},
        value::{Value, ValueKind},
        FstNode,
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
                (0..7).into(),
                vec![],
                Box::new(Expression::new(
                    (2..5).into(),
                    ExpressionKind::Value(Value::new((2..5).into(), ValueKind::Integer(123),)),
                )),
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
                (2..13).into(),
                StatementKind::Let {
                    is_mutable: false,
                    ident: Identifier::new((6..7).into(), "x".into()),
                    expression: Expression::new(
                        (10..11).into(),
                        ExpressionKind::Value(Value::new((10..11).into(), ValueKind::Integer(3))),
                    ),
                },
            )]
        );
        assert_eq!(
            expr.expression(),
            &Expression::new(
                (28..31).into(),
                ExpressionKind::Value(Value::new((28..31).into(), ValueKind::Integer(123))),
            )
        );
    }
}
