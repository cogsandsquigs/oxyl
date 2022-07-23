pub trait Node {
    fn children(&self) -> Vec<Box<dyn Node>>
    where
        Self: Sized;
}

pub struct Literal {}

impl Node for Literal {
    fn children(&self) -> Vec<Box<dyn Node>> {
        return vec![];
    }
}
