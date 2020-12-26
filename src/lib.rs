pub trait Fnum {
    fn variant_count() -> usize;
    fn variant_idx(&self) -> usize;
    unsafe fn uninit_variant(idx: usize) -> Self;
    fn size_of_variant(idx: usize) -> usize;
}
