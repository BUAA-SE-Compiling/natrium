#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    Int,
    Double,
    Void,
}
