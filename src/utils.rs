#[inline]
pub fn x<N: Into<u64>>(value: N, from: usize, size: usize) -> i64 {
    ((value.into() >> from) & ((1 << size) - 1)) as i64
}
