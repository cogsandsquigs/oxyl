pub mod ident;

use std::collections::HashMap;

use ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    /// The map from symbol to whatever.
    sym_map: HashMap<Ident, ()>,

    /// The map from symbol to type.
    type_map: HashMap<Ident, ()>,
}
