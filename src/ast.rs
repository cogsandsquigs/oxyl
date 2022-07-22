pub trait Node {
    fn children() -> Vec<Box<Self>>;
}
