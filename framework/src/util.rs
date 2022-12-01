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
