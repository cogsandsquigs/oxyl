use crate::{
    compile::parser::errors::ParserError,
    fst::{
        expression::{Expression, ExpressionKind, Operator},
        FstNode,
    },
};

pub fn cons_prefix(op: Operator, rhs: Expression) -> std::result::Result<Expression, ParserError> {
    Ok(Expression::new(
        op.location().union_between(*rhs.location()),
        ExpressionKind::Prefix {
            operator: op,
            rhs: Box::new(rhs),
        },
    ))
}

pub fn cons_infix(
    lhs: Expression,
    op: Operator,
    rhs: Expression,
) -> std::result::Result<Expression, ParserError> {
    Ok(Expression::new(
        lhs.location().union_between(*rhs.location()),
        ExpressionKind::Infix {
            operator: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    ))
}

pub fn cons_postfix(lhs: Expression, op: Operator) -> std::result::Result<Expression, ParserError> {
    Ok(Expression::new(
        op.location().union_between(*lhs.location()),
        ExpressionKind::Postfix {
            operator: op,
            lhs: Box::new(lhs),
        },
    ))
}
