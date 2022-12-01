use crate::astr::*;
use colored::Colorize;
use std::{
    fmt::{self, Write},
    ops::Deref,
};

#[derive(Debug)]
pub struct ColoredOutput {
    value: String,
    control_count: usize,
}

impl ColoredOutput {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn control_count(&self) -> usize {
        self.control_count
    }
}

impl fmt::Display for ColoredOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <String as fmt::Display>::fmt(&self.value, f)
    }
}

auto trait NotIntoColorOutput {}
impl !NotIntoColorOutput for ColoredOutput {}
impl !NotIntoColorOutput for AString {}
impl<'s> !NotIntoColorOutput for &'s AStr {}

impl<T: fmt::Display + NotIntoColorOutput> From<T> for ColoredOutput {
    fn from(value: T) -> Self {
        value.to_string().into()
    }
}

impl From<String> for ColoredOutput {
    fn from(value: String) -> Self {
        let before_style_len = value.len();
        let value = value.white().bold().to_string();
        let control_count = value.len() - before_style_len;
        ColoredOutput {
            value,
            control_count,
        }
    }
}

impl From<AString> for ColoredOutput {
    fn from(s: AString) -> Self {
        s.as_slice().into()
    }
}

impl<'s> From<&'s AStr> for ColoredOutput {
    fn from(s: &'s AStr) -> Self {
        String::from_utf8_lossy(s).deref().into()
    }
}

macro_rules! impl_binary_op_output {
    ($symbol:literal, $struct_name:ident, $trait_name:ident, $trait_fn:ident, $identity_trait:ident, $identity_fn:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $struct_name<T>(pub T);

        impl<T> !NotIntoColorOutput for $struct_name<T> {}
        impl<T, I> From<$struct_name<T>> for ColoredOutput
        where
            T: IntoIterator<Item = I>,
            I: fmt::Display + std::ops::$trait_name<Output = I> + num::$identity_trait,
        {
            fn from(input: $struct_name<T>) -> Self {
                let mut value = String::new();
                let mut str_len = 0;
                let mut acc: I = num::$identity_fn();
                for (i, v) in input.0.into_iter().enumerate() {
                    let s = v.to_string();
                    if i != 0 {
                        _ = write!(value, "{} ", $symbol.bright_magenta());
                        str_len += 2;
                    }
                    _ = write!(value, "{} ", s.white());
                    acc = acc.$trait_fn(v);
                    str_len += s.len() + 1;
                }

                if str_len != 0 {
                    _ = write!(value, "{} ", "=".bright_magenta());
                    str_len += 2;
                }

                let s = acc.to_string();
                _ = write!(value, "{}", s.white().bold());
                str_len += s.len();
                let control_count = value.len() - str_len;
                ColoredOutput {
                    value,
                    control_count,
                }
            }
        }
    };
}

impl_binary_op_output!("+", AddOutput, Add, add, Zero, zero);
impl_binary_op_output!("-", SubOutput, Sub, sub, Zero, zero);
impl_binary_op_output!("*", MulOutput, Mul, mul, One, one);
impl_binary_op_output!("/", DivOutput, Div, div, One, one);
