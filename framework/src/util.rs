use std::mem::{ManuallyDrop, MaybeUninit};

pub fn init_array<T, const N: usize, F: FnMut(usize) -> T>(mut f: F) -> [T; N] {
    struct InitState<T, const N: usize> {
        array: ManuallyDrop<[MaybeUninit<T>; N]>,
        index: usize,
    }
    impl<T, const N: usize> Drop for InitState<T, N> {
        fn drop(&mut self) {
            for i in 0..self.index {
                unsafe { self.array[i].assume_init_drop() }
            }
        }
    }
    let mut state = InitState::<T, N> {
        array: ManuallyDrop::new(MaybeUninit::uninit_array()),
        index: 0,
    };
    while state.index < N {
        state.array[state.index].write(f(state.index));
        state.index += 1;
    }

    unsafe { MaybeUninit::array_assume_init(ManuallyDrop::take(&mut state.array)) }
}

pub trait VecExt<T> {
    fn get_two_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)>;
}

impl<T> VecExt<T> for Vec<T> {
    fn get_two_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)> {
        if a >= self.len() || b >= self.len() {
            return None;
        }
        use std::cmp::Ordering::*;
        match a.cmp(&b) {
            Less => {
                let (n, m) = self.split_at_mut(b);
                Some((&mut n[a], &mut m[0]))
            }
            Equal => None,
            Greater => {
                let (n, m) = self.split_at_mut(a);
                Some((&mut m[0], &mut n[b]))
            }
        }
    }
}
