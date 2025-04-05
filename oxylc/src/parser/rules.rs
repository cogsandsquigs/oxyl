use crate::{
    ast::{
        block::Block, expression::Expression, file::File, function::Function,
        identifier::Identifier, statement::Statement, value::Value, Node, Span,
    },
    parser::Rule,
};
use pest::iterators::Pair;

/// Parses a file rule.
pub fn file(pair: Pair<'_, Rule>) -> Node<File> {
    // NOTE: SOI rule does not produce a token, so no need to check for it.

    let span: Span = (&pair).into();
    let mut pairs = pair.into_inner();

    println!("Parsing: {:?}", pairs);

    let mut stmts = vec![];

    for pair in pairs.by_ref() {
        if pair.as_rule() == Rule::EOI {
            break;
        }

        stmts.push(statement(pair));
    }

    Node::new(span, File::new(stmts))
}

/// Parses a statement rule.
pub fn statement(pair: Pair<'_, Rule>) -> Node<Statement> {
    let span: Span = (&pair).into();
    match pair.as_rule() {
        Rule::LET_STATEMENT => {
            let mut pairs = pair.into_inner();
            let name = identifier(pairs.next().expect("The name of the variable is required"));
            let expr = expression(pairs.next().expect("The expression is required"));
            Node::new(span, Statement::Let { name, expr })
        }
        _ => unreachable!(),
    }
}

/// Parses an expression rule.
pub fn expression(pair: Pair<'_, Rule>) -> Node<Expression> {
    todo!()
}

/// Parses a Value rule.
pub fn value(pair: Pair<'_, Rule>) -> Node<Value> {
    let span: Span = (&pair).into();
    match pair.as_rule() {
        Rule::INTEGER => Node::new(
            span,
            Value::Integer(pair.as_str().parse().expect("Numbers must parse")),
        ),
        Rule::IDENTIFIER => Node::new(span, Value::Identifier(identifier(pair))),
        Rule::FUNCTION => Node::new(span, Value::Function(function(pair))),
        Rule::BLOCK => Node::new(span, Value::Block(block(pair))),
        _ => unreachable!(),
    }
}

/// Parses a Block rule.
pub fn block(pair: Pair<'_, Rule>) -> Node<Block> {
    let span: Span = (&pair).into();
    let mut pairs = pair.into_inner();
    let mut stmts = vec![];

    while pairs.peek().map(|p| p.as_rule()) != Some(Rule::STATEMENT) {
        let value = pairs.next().unwrap();
        stmts.push(statement(value));
    }

    Node::new(
        span,
        Block::new(
            stmts,
            expression(
                pairs
                    .next()
                    .expect("Expected an expression after block statements"),
            ),
        ),
    )
}

/// Parses a Function rule.
pub fn function(pair: Pair<'_, Rule>) -> Node<Function> {
    let span: Span = (&pair).into();
    let mut pairs = pair.into_inner();
    let mut args = vec![];

    while pairs.peek().map(|p| p.as_rule()) == Some(Rule::IDENTIFIER) {
        let arg = pairs.next().unwrap();
        args.push(identifier(arg));
    }

    let expr = expression(
        pairs
            .next()
            .expect("Expected an expression after function arguments"),
    );

    Node::new(span, Function::new(args, expr))
}

/// Parses an Identifier rule.
/// NOTE: Assumes that `pair.as_rule()` is `Rule::IDENTIFIER`.
pub fn identifier(pair: Pair<'_, Rule>) -> Node<Identifier> {
    Node::new(&pair, Identifier::new(pair.as_str()))
}
