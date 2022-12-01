pub trait IteratorExt: Iterator {
    /// Finds the item that is not the same as all the others in the collection
    fn find_distinct(&mut self) -> DistinctResult<Self::Item>
    where
        Self::Item: Eq,
    {
        use DistinctResult::*;
        let first = match self.next() {
            Some(v) => v,
            None => return TooFewElements,
        };
        let second = match self.next() {
            Some(v) => v,
            None => return TooFewElements,
        };
        let (mut index, common, mut distinct) = if first == second {
            (1, first, None)
        } else {
            let third = match self.next() {
                Some(v) => v,
                None => return TooFewElements,
            };
            if first == third {
                (2, first, Some((1, second)))
            } else if second == third {
                (2, second, Some((0, first)))
            } else {
                return MultipleDistinct;
            }
        };

        for value in self {
            index += 1;
            if value == common {
                continue;
            }
            if distinct.is_some() {
                return MultipleDistinct;
            }
            distinct = Some((index, value));
        }

        match distinct {
            Some((index, distinct)) => SingleDistinct(Distinct {
                index,
                common,
                distinct,
            }),
            None => Unique(common),
        }
    }
}

impl<T: Iterator> IteratorExt for T {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistinctResult<T> {
    /// There less than two elements, or exactly two distinct elements.
    TooFewElements,
    /// All values are the same.
    Unique(T),
    /// There is a single distinct value.
    SingleDistinct(Distinct<T>),
    /// There are multiple distinct values.
    MultipleDistinct,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Distinct<T> {
    pub index: usize,
    pub common: T,
    pub distinct: T,
}
