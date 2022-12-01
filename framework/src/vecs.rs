use std::fmt;

macro_rules! substitute {
    ($name:ident, $($token:tt)+) => {
        $($token)+
    };
}

macro_rules! impl_vec {
    ($name:ident, $($component:ident),+, $str_fmt:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name<T> {
            $(pub $component: T,)+
        }

        impl<T> $name<T> {
            pub fn new($($component: T),+) -> Self {
                $name { $($component,)+ }
            }
        }

        impl<T> From<($(substitute!($component, T),)+)> for $name<T> {
            fn from(($($component,)+): ($(substitute!($component, T),)+)) -> Self {
                $name { $($component,)+ }
            }
        }

        impl<T: num::Zero> $name<T> {
            pub fn zero() -> Self {
                $name {
                    $($component: num::zero(),)+
                }
            }
        }

        macro_rules! impl_binary_op {
            ($trait:ident, $fn_name:ident, $assign_trait:ident, $assign_fn_name:ident) => {
                impl<T: std::ops::$trait<Rhs, Output = O>, Rhs, O> std::ops::$trait<$name<Rhs>>
                    for $name<T>
                {
                    type Output = $name<O>;

                    fn $fn_name(self, rhs: $name<Rhs>) -> Self::Output {
                        $name {
                            $($component: self.$component.$fn_name(rhs.$component),)+
                        }
                    }
                }

                impl<T: std::ops::$assign_trait<Rhs>, Rhs> std::ops::$assign_trait<$name<Rhs>>
                    for $name<T>
                {
                    fn $assign_fn_name(&mut self, rhs: $name<Rhs>) {
                        $(self.$component.$assign_fn_name(rhs.$component);)+
                    }
                }
            };
        }

        macro_rules! impl_unary_op {
            ($trait:ident, $fn_name:ident) => {
                impl<T: std::ops::$trait<Output = O>, O> std::ops::$trait for $name<T> {
                    type Output = $name<O>;
                    fn $fn_name(self) -> $name<O> {
                        $name {
                            $($component: self.$component.$fn_name(),)+
                        }
                    }
                }
            };
        }

        impl_binary_op!(Add, add, AddAssign, add_assign);
        impl_binary_op!(Sub, sub, SubAssign, sub_assign);
        impl_binary_op!(Mul, mul, MulAssign, mul_assign);
        impl_binary_op!(Div, div, DivAssign, div_assign);
        impl_binary_op!(Rem, rem, RemAssign, rem_assign);
        impl_binary_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
        impl_binary_op!(BitOr, bitor, BitOrAssign, bitor_assign);
        impl_binary_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);
        impl_binary_op!(Shl, shl, ShlAssign, shl_assign);
        impl_binary_op!(Shr, shr, ShrAssign, shr_assign);

        impl_unary_op!(Neg, neg);
        impl_unary_op!(Not, not);

        impl<T: fmt::Display> fmt::Display for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $str_fmt, $(self.$component),+)
            }
        }
    };
}

impl_vec!(Vec1, x, "({})");
impl_vec!(Vec2, x, y, "({}, {})");
impl_vec!(Vec3, x, y, z, "({}, {}, {})");
impl_vec!(Vec4, x, y, z, w, "({}, {}, {}, {})");
