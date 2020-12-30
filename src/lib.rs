extern crate fnum_derive;
extern crate once_cell;

pub use fnum_derive::Fnum;
pub use once_cell::sync::Lazy as __Lazy;

pub trait Fnum {
    fn variant_count() -> usize;
    fn variant_index(&self) -> usize;
    unsafe fn uninit_variant(idx: usize) -> Self;
    fn size_of_variant(idx: usize) -> usize;
}
