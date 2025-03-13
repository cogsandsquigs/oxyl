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
        value::{Value, ValueKind},
    };

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
}
