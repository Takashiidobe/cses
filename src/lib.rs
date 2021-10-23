macro_rules! test {
    ($($name:ident: $left:expr, $right:expr,)*) => {
        #[cfg(test)]
        mod test {
        use super::*;
            $(
                #[test]
                fn $name() {
                    assert_eq!($left, $right);
                }
            )*
        }
    }
}

pub mod introductory;
pub mod sorting_and_searching;
