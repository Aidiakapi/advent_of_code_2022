use crate::astr::AStr;
use std::slice::Split;

pub trait AStrExt {
    fn lines(&self) -> Split<'_, u8, for<'r> fn(&'r u8) -> bool>;
}

pub struct Lines<'s>(&'s AStr);

impl AStrExt for AStr {
    fn lines(&self) -> Split<'_, u8, for<'r> fn(&'r u8) -> bool> {
        self.split(|&l| l == b'\n')
    }
}
