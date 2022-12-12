use crate::parsers::ParseError;
use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

type Vec2 = crate::vecs::Vec2<usize>;

/// A 2D grid
pub trait Grid<T>: Sized + IndexMut<Self::Indexer> {
    type Indexer;
    type Builder: GridBuilder<T, Output = Self>;
}

pub trait GridBuilder<T> {
    type Output;
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn push_cell(&mut self, cell: T) -> Result<(), ParseError>;
    fn advance_next_line(&mut self) -> Result<(), ParseError>;
    fn finish(self) -> Result<Self::Output, ParseError>;
}

#[derive(Debug, Clone)]
pub struct VecGrid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct VecGridBuilder<T> {
    width: Option<usize>,
    x: usize,
    data: Vec<T>,
}

impl<T> Grid<T> for VecGrid<T> {
    type Indexer = Vec2;
    type Builder = VecGridBuilder<T>;
}

impl<T> VecGrid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get<V: Into<Vec2>>(&self, index: V) -> Option<&T> {
        let index = index.into();
        if index.x < self.width && index.y < self.height {
            unsafe { Some(self.get_unchecked(index)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut<V: Into<Vec2>>(&mut self, index: V) -> Option<&mut T> {
        let index = index.into();
        if index.x < self.width && index.y < self.height {
            unsafe { Some(self.get_unchecked_mut(index)) }
        } else {
            None
        }
    }

    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: Vec2) -> &T {
        unsafe { self.data.get_unchecked(index.y * self.width + index.x) }
    }

    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: Vec2) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index.y * self.width + index.x) }
    }

    #[inline]
    pub fn cells(&self) -> &[T] {
        &self.data
    }

    #[inline]
    pub fn cells_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    pub fn iter(&self) -> VecGridIter<'_, T> {
        VecGridIter {
            data: self.data.iter(),
            width: self.width,
            height: self.height,
            next: Vec2::zero(),
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> VecGridIterMut<'_, T> {
        VecGridIterMut {
            data: self.data.iter_mut(),
            width: self.width,
            height: self.height,
            next: Vec2::zero(),
        }
    }
}

impl<T, V: Into<Vec2>> Index<V> for VecGrid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: V) -> &Self::Output {
        let index = index.into();
        assert!(index.x < self.width);
        assert!(index.y < self.height);
        unsafe { self.data.get_unchecked(index.y * self.width + index.x) }
    }
}
impl<T, V: Into<Vec2>> IndexMut<V> for VecGrid<T> {
    #[inline]
    fn index_mut(&mut self, index: V) -> &mut Self::Output {
        let index = index.into();
        assert!(index.x < self.width);
        assert!(index.y < self.height);
        unsafe { self.data.get_unchecked_mut(index.y * self.width + index.x) }
    }
}

impl<T> GridBuilder<T> for VecGridBuilder<T> {
    type Output = VecGrid<T>;
    fn new() -> Self {
        VecGridBuilder {
            width: None,
            x: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push_cell(&mut self, cell: T) -> Result<(), ParseError> {
        if let Some(width) = self.width {
            if self.x >= width {
                return Err(ParseError::GridCellAfterEndOfRowReached);
            }
        }
        self.data.push(cell);
        self.x += 1;
        Ok(())
    }

    fn advance_next_line(&mut self) -> Result<(), ParseError> {
        if let Some(width) = self.width {
            if self.x != width {
                return Err(ParseError::GridIncompleteRow);
            }
        } else {
            self.width = Some(self.x);
        }
        self.x = 0;
        Ok(())
    }

    fn finish(mut self) -> Result<Self::Output, ParseError> {
        if self.width.is_none() {
            self.advance_next_line()?;
        }
        let width = self.width.unwrap();
        if self.x != 0 && self.x != width {
            return Err(ParseError::GridIncompleteRow);
        }
        debug_assert!(self.data.len() % width == 0);
        let height = self.data.len() / width;
        Ok(VecGrid {
            width,
            height,
            data: self.data,
        })
    }
}

impl<T> IntoIterator for VecGrid<T> {
    type IntoIter = VecGridIntoIter<T>;
    type Item = (Vec2, T);

    fn into_iter(self) -> Self::IntoIter {
        VecGridIntoIter {
            data: self.data.into_iter(),
            width: self.width,
            height: self.height,
            next: Vec2::zero(),
        }
    }
}

pub struct VecGridIntoIter<T> {
    data: <Vec<T> as IntoIterator>::IntoIter,
    width: usize,
    height: usize,
    next: Vec2,
}

pub struct VecGridIter<'g, T> {
    data: Iter<'g, T>,
    width: usize,
    height: usize,
    next: Vec2,
}

pub struct VecGridIterMut<'g, T> {
    data: IterMut<'g, T>,
    width: usize,
    height: usize,
    next: Vec2,
}

macro impl_iter() {
    fn next(&mut self) -> Option<Self::Item> {
        let point = self.next;
        if point.y >= self.height {
            return None;
        }
        let item = unsafe { self.data.next().unwrap_unchecked() };
        self.next.x += 1;
        if self.next.x >= self.width {
            self.next.x = 0;
            self.next.y += 1;
        }
        return Some((point, item));
    }
}

impl<T> Iterator for VecGridIntoIter<T> {
    type Item = (Vec2, T);
    impl_iter!();
}

impl<'g, T> Iterator for VecGridIter<'g, T> {
    type Item = (Vec2, &'g T);
    impl_iter!();
}

impl<'g, T> Iterator for VecGridIterMut<'g, T> {
    type Item = (Vec2, &'g mut T);
    impl_iter!();
}
