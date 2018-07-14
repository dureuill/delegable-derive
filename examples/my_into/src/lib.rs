#![feature(proc_macro)]

extern crate delegable_derive;

use delegable_derive::delegable;

#[delegable]
pub trait MyInto<T> {
    fn into(self) -> T;
}

#[cfg(test)]
mod tests {

    use super::*;

    struct Convertible {
        x: u64,
    }
    impl MyInto<u64> for Convertible {
        fn into(self) -> u64 {
            self.x
        }
    }

    struct ConvertibleProxy {
        c: Convertible,
    }

    impl delegate_MyInto for ConvertibleProxy {
        type Inner = Convertible;
        type gen_T = u64;

        fn inner(&self) -> &Self::Inner {
            &self.c
        }
        fn inner_mut(&mut self) -> &mut Self::Inner {
            &mut self.c
        }
        fn into_inner(self) -> Self::Inner {
            self.c
        }
        fn from_inner(delegate: Self::Inner) -> Self {
            Self { c: delegate }
        }
    }

    #[test]
    fn test_convert() {
        let c = ConvertibleProxy {
            c: Convertible { x: 42 },
        };
        assert_eq!(MyInto::into(c), 42);
    }
}
