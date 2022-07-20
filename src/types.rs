#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Type {
    // standard types
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}
