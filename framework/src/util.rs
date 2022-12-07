use std::mem::MaybeUninit;

pub fn init_array<T, E, const N: usize, F: FnMut(usize) -> Result<T, E>>(
    mut f: F,
) -> Result<[T; N], E> {
    struct DropGuard<'r, T, const N: usize> {
        result: &'r mut [MaybeUninit<T>; N],
        initialized_count: usize,
    }

    impl<T, const N: usize> Drop for DropGuard<'_, T, N> {
        fn drop(&mut self) {
            for i in (0..self.initialized_count).rev() {
                unsafe {
                    self.result[i].assume_init_drop();
                }
            }
        }
    }

    let mut result = MaybeUninit::<T>::uninit_array::<N>();
    let mut drop_guard = DropGuard {
        result: &mut result,
        initialized_count: 0,
    };

    for i in 0..N {
        drop_guard.result[i].write(f(i)?);
        drop_guard.initialized_count += 1;
    }

    std::mem::forget(drop_guard);
    Ok(unsafe { MaybeUninit::array_assume_init(result) })
}

pub trait SliceExt<T> {
    fn get_two_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)>;
}

impl<T> SliceExt<T> for [T] {
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
