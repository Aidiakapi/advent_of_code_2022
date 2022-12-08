use crate::{parsers::ParseError, vecs::Vec2};
use std::ops::{Index, IndexMut};

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
    type Indexer = Vec2<usize>;
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
    pub fn get<V: Into<Vec2<usize>>>(&self, index: V) -> Option<&T> {
        let index = index.into();
        if index.x < self.width && index.y < self.height {
            unsafe { Some(self.data.get_unchecked(index.y * self.width + index.x)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut<V: Into<Vec2<usize>>>(&mut self, index: V) -> Option<&mut T> {
        let index = index.into();
        if index.x < self.width && index.y < self.height {
            unsafe { Some(self.data.get_unchecked_mut(index.y * self.width + index.x)) }
        } else {
            None
        }
    }
}

impl<T, V: Into<Vec2<usize>>> Index<V> for VecGrid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: V) -> &Self::Output {
        let index = index.into();
        assert!(index.x < self.width);
        assert!(index.y < self.height);
        unsafe { self.data.get_unchecked(index.y * self.width + index.x) }
    }
}
impl<T, V: Into<Vec2<usize>>> IndexMut<V> for VecGrid<T> {
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
