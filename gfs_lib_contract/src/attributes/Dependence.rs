
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dependence {
    Owned,
    Referenced
}