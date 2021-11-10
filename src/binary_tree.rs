#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<V: PartialOrd> {
  pub value: V,
  pub left: Option<Box<Node<V>>>,
  pub right: Option<Box<Node<V>>>,
}