use r0syntax::util::P;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ty {
    Int,
    Double,
    Bool,
    Func(FuncTy),
    Void,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncTy {
    pub params: Vec<P<Ty>>,
    pub ret: P<Ty>,
}

impl Ty {
    pub fn size(&self) -> usize {
        match self {
            Ty::Int | Ty::Double => 8,
            Ty::Bool => 1,
            Ty::Func(_) => 0,
            Ty::Void => 0,
        }
    }

    pub fn size_slot(&self) -> usize {
        match self {
            Ty::Int | Ty::Double | Ty::Bool => 1,
            Ty::Func(_) => 0,
            Ty::Void => 0,
        }
    }

    pub fn get_func(&self) -> Option<&FuncTy> {
        match self {
            Ty::Func(f) => Some(f),
            _ => None,
        }
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Int => write!(f, "int"),
            Ty::Double => write!(f, "double"),
            Ty::Bool => write!(f, "bool"),
            Ty::Func(ty) => {
                write!(f, "Fn(")?;
                let mut param_iter = ty.params.iter();

                if let Some(r) = param_iter.next() {
                    write!(f, "{}", r)?;
                }
                for r in param_iter {
                    write!(f, ", {}", r)?;
                }

                write!(f, ") -> {}", ty.ret)
            }
            Ty::Void => write!(f, "void"),
        }
    }
}
