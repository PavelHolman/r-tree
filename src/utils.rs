use generic_array::{ArrayLength, GenericArray};

pub(crate) trait Unzip<T1, T2, N>
    where N: ArrayLength {
    fn unzip(self) -> (GenericArray<T1, N>, GenericArray<T2, N>);
}

impl<T1, T2, N> Unzip<T1, T2, N> for GenericArray<(T1, T2), N>
    where T1: Copy, T2: Copy, N: ArrayLength {
    fn unzip(self) -> (GenericArray<T1, N>, GenericArray<T2, N>) {
        let mut first = GenericArray::uninit();
        let mut second = GenericArray::uninit();
        for i in 0..N::to_usize() {
            first[i].write(self[i].0);
            second[i].write(self[i].1);
        }

        (unsafe { GenericArray::assume_init(first) }, unsafe { GenericArray::assume_init(second) })
    }
}
