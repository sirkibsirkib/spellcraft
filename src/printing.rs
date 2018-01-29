
use std::fmt;
use magic::*;

impl fmt::Debug for ESlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E_{}", self.0)
    }
}

impl fmt::Debug for ESetSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eset_{}", self.0)
    }
}


impl fmt::Debug for DSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "D_{}", self.0)
    }
}

impl fmt::Debug for LSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L_{}", self.0)
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
            &LastOf(ref a) => {
                fmt.debug_tuple("LastOf")
                .field(&a).finish()
            },
        }
    }
}