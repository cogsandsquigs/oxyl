use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    // standard types
    None,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Box<Type>, Vec<Type>),
    Struct(&'static str, HashMap<&'static str, Type>),
}

impl Type {
    /**
     * Returns true if the two types are the same.
     * This is used to compare types in the type checker.
     * This does NOT compare the values of the types.
     */
    pub fn same(a: Type, b: Type) -> bool {
        match (a, b) {
            (Type::None, Type::None) => true,
            (Type::Int(_), Type::Int(_)) => true,
            (Type::Float(_), Type::Float(_)) => true,
            (Type::String(_), Type::String(_)) => true,
            (Type::Bool(_), Type::Bool(_)) => true,
            // Checks if types are the same by checking the type of the
            // elements (the first value in the tuple). This assumes that
            // all of the array values are of the same type. Make sure to
            // assert the same type for all of the array values before
            // inserting them
            // TODO: maybe make this more efficient?
            (Type::Array(a, b), Type::Array(c, d)) => {
                return Type::same(*a, *c)
                    && b.iter()
                        .zip(d.iter())
                        .all(|(x, y)| Type::same(x.clone(), y.clone()))
            }
            // Checks if types are the same by making sure that the struct
            // names are the same, and then comparing the fields to see if
            // they are the same.
            // TODO: maybe make this more efficient?
            (Type::Struct(a, b), Type::Struct(c, d)) => {
                return a == c
                    && b.keys().len() == d.keys().len()
                    && b.iter()
                        .zip(d.iter())
                        .all(|(x, y)| x.0 == y.0 && Type::same(x.1.clone(), y.1.clone()));
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests if the `same` function works correctly, by testing ALL
     * values in ALL possible combinations of types.
     */
    #[test]
    fn test_same() {
        assert!(Type::same(Type::None, Type::None));
        assert!(!Type::same(Type::None, Type::Int(0)));
        assert!(Type::same(Type::Int(0), Type::Int(0)));
        assert!(Type::same(Type::Int(0), Type::Int(1)));
        assert!(Type::same(Type::Float(0.0), Type::Float(0.0)));
        assert!(Type::same(Type::Float(0.0), Type::Float(0.1)));
        assert!(Type::same(
            Type::String("".into()),
            Type::String("testing!".into())
        ));
        assert!(Type::same(Type::Bool(true), Type::Bool(false)));
        assert!(Type::same(
            Type::Array(Box::new(Type::Int(0)), vec![]),
            Type::Array(Box::new(Type::Int(0)), vec![Type::Int(0)])
        ));
        assert!(!Type::same(
            Type::Array(Box::new(Type::Int(0)), vec![]),
            Type::Array(Box::new(Type::Float(0.0)), vec![])
        ));
        // test structs
        assert!(Type::same(
            Type::Struct("test", HashMap::from_iter(vec![("t", Type::Int(0))])),
            Type::Struct("test", HashMap::from_iter(vec![("t", Type::Int(0))]))
        ));
        assert!(!Type::same(
            Type::Struct("test", HashMap::from_iter(vec![("t", Type::Int(0))])),
            Type::Struct("test", HashMap::from_iter(vec![("t", Type::Float(0.0))]))
        ));
    }
}
