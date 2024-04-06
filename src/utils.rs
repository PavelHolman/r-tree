pub(crate) trait Unzip<T1, T2, const N: usize> {
    fn unzip(self) -> ([T1; N], [T2; N]);
}

impl<T1, T2, const N: usize> Unzip<T1, T2, N> for [(T1, T2); N] {
    fn unzip(self) -> ([T1; N], [T2; N]) {
        unzip_array_of_tuple::unzip_array_of_tuple(self)
    }
}
