/// An identifier. This is used for variable names, function names, etc. Note that instead of using
/// the actual "name", we use a number to represent the identifier. This is because we want to make
/// everything as `Copy`-able as possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident {
    inner: u64,
}
