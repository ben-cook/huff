use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<V: PartialOrd + Debug> {
  pub value: V,
  pub left: Option<Box<Node<V>>>,
  pub right: Option<Box<Node<V>>>,
}

impl<V: PartialOrd + Debug> Display for Node<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}