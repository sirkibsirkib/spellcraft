
use std::fmt;
use magic::*;

impl fmt::Debug for F32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Debug for ESlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E_{:?}", self.0)
    }
}

impl fmt::Debug for ESetSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eset_{:?}", self.0)
    }
}


impl fmt::Debug for DSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "D_{:?}", self.0)
    }
}

impl fmt::Debug for LSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L_{:?}", self.0)
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Entity::*;
        // let tmp = fmt.debug_tuple("Entity");
        match self {
            &LoadEntity(ref x) => write!(fmt, "LoadEntity({:?})", x),
            &FirstOf(ref a) => {
                fmt.debug_tuple("FirstOf")
                .field(&a).finish()
            },
            &Choose(ref a) => {
                fmt.debug_tuple("Choose")
                .field(&a).finish()
            },
            &ClosestFrom(ref a, ref b) => {
                fmt.debug_tuple("ClosestFrom")
                .field(&a).field(&b).finish()
            },
            // &LastOf(ref a) => {
            //     fmt.debug_tuple("LastOf")
            //     .field(&a).finish()
            // },
        }
    }
}

impl fmt::Debug for Discrete {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Discrete::*;
        match self {
            &Const(ref x) => write!(fmt, "Const({:?})", x),
            &Range(ref x, ref y) => write!(fmt, "Range({:?},{:?})", x, y),
            &WithinPercent(ref x, ref y) => write!(fmt, "WithinPercent({:?},{:?})", x, y),
            &Div(ref a, ref b) => fmt.debug_tuple("Div").field(a).field(b).finish(),
            &Sum(ref a) => fmt.debug_tuple("Sum") .field(a).finish(),
            &Neg(ref a) => fmt.debug_tuple("Neg") .field(a).finish(),
            &Mult(ref a) => fmt.debug_tuple("Mult") .field(a).finish(),
            &Max(ref a) => fmt.debug_tuple("Max") .field(a).finish(),
            &Min(ref a) => fmt.debug_tuple("Min") .field(a).finish(),
            &CountStacks(ref a, ref b) => fmt.debug_tuple("CountStacks").field(a).field(b).finish(),
            &CountDur(ref a, ref b) => fmt.debug_tuple("CountDur").field(a).field(b).finish(),
            &Choose(ref a) => fmt.debug_tuple("Choose") .field(a).finish(),
            &Cardinality(ref a) => fmt.debug_tuple("Cardinality") .field(a).finish(),
            &LoadFrom(ref x) => write!(fmt, "LoadFrom({:?})", x),
        }
    }
}

impl fmt::Debug for Definition {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Definition::*;
        match self {
            &ESet(ref a, ref b) => fmt.debug_tuple(&format!("{:?}", a)).field(b).finish(),
            &E(ref a, ref b) => fmt.debug_tuple(&format!("{:?}", a)).field(b).finish(),
            &D(ref a, ref b) => fmt.debug_tuple(&format!("{:?}", a)).field(b).finish(),
            &L(ref a, ref b) => fmt.debug_tuple(&format!("{:?}", a)).field(b).finish(),
        }
    }
}