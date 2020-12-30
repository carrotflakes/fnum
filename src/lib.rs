//! Fnum is an utility trait for enum, that provides [`Fnum::size_of_variant`] method.
//! [`Fnum::size_of_variant`] returns the minimum size that specified variant requiring.
//!
//! ```
//! use fnum::Fnum;
//!
//! #[derive(Fnum)]
//! enum MyEnum {
//!     A(u64),
//!     B(String),
//!     C(u64, u32, u32, u32),
//!     D {
//!         foo: u32,
//!         bar: String,
//!     }
//! }
//!
//! assert_eq!(MyEnum::size_of_variant(0), 16); // bytes required by `MyEnum::A(..)`
//! assert_eq!(MyEnum::size_of_variant(1), 32); // bytes required by `MyEnum::B(..)`
//! assert_eq!(MyEnum::size_of_variant(2), 24); // bytes required by `MyEnum::C(..)`
//! assert_eq!(MyEnum::size_of_variant(3), 32); // bytes required by `MyEnum::D{..}`
//! ```

extern crate fnum_derive;
extern crate once_cell;

pub use fnum_derive::Fnum;
pub use once_cell::sync::Lazy as __Lazy;

/// Fnum trait providing [`Fnum::size_of_variant`] method.
pub trait Fnum {
    /// Returns the number of variants the enum has.
    fn variant_count() -> usize;

    /// Returns the index of variant.
    ///
    /// ```
    /// #[derive(Fnum)]
    /// enum MyEnum {
    ///     FIrst,
    ///     Second,
    /// }
    ///
    /// assert_eq!(MyEnum::First.variant_index(), 0);
    /// assert_eq!(MyEnum::Second.variant_index(), 1);
    /// ```
    fn variant_index(&self) -> usize;

    /// Create an enum value that has uninitialized fields.
    ///
    /// DO NOT drop the returned value, use [`std::mem::forget`].
    unsafe fn uninit_variant(idx: usize) -> Self;

    /// Returns the minimum size that specified variant requiring.
    fn size_of_variant(idx: usize) -> usize;
}
