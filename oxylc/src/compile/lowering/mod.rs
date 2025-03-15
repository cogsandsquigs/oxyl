use crate::fst::{
    block::Block,
    expression::{Expression, ExpressionKind},
    function::Function,
    identifier::Identifier,
    statement::{Statement, StatementKind},
    value::{Value, ValueKind},
    visitor::FstVisitor,
    File,
};

pub struct CLowerer;

impl CLowerer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl FstVisitor<String> for CLowerer {
    fn visit_file(&mut self, file: &File) -> String {
        file.statements()
            .iter()
            .map(|statement| self.visit_statement(statement))
            .reduce(|a, b| a + "\n" + &b)
            .unwrap_or("".to_string())
    }

    fn visit_statement(&mut self, statement: &Statement) -> String {
        match statement.kind() {
            StatementKind::Let {
                is_mutable,
                ident,
                expression,
            } => self.visit_statement_let(is_mutable, ident, expression),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> String {
        match expression.kind() {
            ExpressionKind::Value(v) => self.visit_value(v),
            ExpressionKind::Block(b) => self.visit_block(b),
            ExpressionKind::Parenthesized { inner, .. } => self.visit_expression(inner),
        }
    }

    fn visit_value(&mut self, value: &Value) -> String {
        match value.kind() {
            ValueKind::Integer(i) => i.to_string(),
            ValueKind::Floating(f) => f.to_string(),
            ValueKind::Boolean(b) => b.to_string(),
            ValueKind::Identifier(i) => self.visit_ident(i),
            ValueKind::Function(f) => self.visit_function(f),
        }
    }

    fn visit_ident(&mut self, ident: &Identifier) -> String {
        ident.name().to_string()
    }

    fn visit_function(&mut self, function: &Function) -> String {
        format!(
            "({}) {}",
            function
                .args()
                .iter()
                .map(|x| self.visit_ident(x))
                .reduce(|a, b| a + ", " + &b)
                .unwrap_or("".into()),
            self.visit_expression(function.expression())
        )
    }

    fn visit_block(&mut self, block: &Block) -> String {
        "{\n".to_string()
            + &block
                .statements()
                .iter()
                .map(|statement| self.visit_statement(statement))
                .reduce(|a, b| a + "\n" + &b)
                .unwrap_or("".to_string())
            + "\nreturn "
            + &self.visit_expression(block.expression())
            + ";\n}"
    }
}

// Assisting functions
impl CLowerer {
    fn visit_statement_let(
        &mut self,
        _is_mutable: &bool,
        ident: &Identifier,
        expression: &Expression,
    ) -> String {
        if matches!(
            expression.kind(),
            ExpressionKind::Value(v)
            if matches!(v.kind(), ValueKind::Function(_))
        ) {
            format!(
                "extern int {} {}\n",
                self.visit_ident(ident),
                self.visit_expression(expression)
            )
        } else {
            format!(
                "int {} = {};\n",
                self.visit_ident(ident),
                self.visit_expression(expression)
            )
        }
    }
}
