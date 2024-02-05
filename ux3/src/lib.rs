#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg

//! # uX3: A better [uX](https://github.com/rust-ux/uX)/[ux2](https://github.com/JonathanWoollett-Light/ux2)
//!
//! [![Crates.io](https://img.shields.io/crates/v/ux3)](https://crates.io/crates/ux3)
//! [![docs](https://img.shields.io/crates/v/ux3?color=yellow&label=docs)](https://docs.rs/ux3)
//!
//! #### Non-standard integer types like `u7`, `u9`, `u10`, `u63`, `i7`, `i9` etc.
//!
//! When non-standard-width integers are required in an application, the norm is to use a larger
//! container and make sure the value is within range after manipulation. uX2 aims to take care of
//! this once and for all by providing `u1`-`u127` and `i1`-`i127` types (depending on the enabled
//! features) that offer safe arithmetic operations.
//!
//! `<core::primitive::i32 as core::ops::Add<core::primitive::i32>>::add` can panic in `Debug` or
//! overflow in `Release`, `<ux3::i32 as core::ops::Add<ux3::i32>>::add` cannot panic or overflow in
//! `Debug` or `Release`, this is because it returns `ux3::i33`. This is applied for all operations
//! and combinations of types in `ux3`. This allows for more thorough compile time type checking.
//!
//!
//! uX3 types take up only as much space as the smallest integer type that can contain them.
//!
//! ## Why does this exist? Why use this over `ux`?
//!
//! I noticed [uX](https://github.com/rust-ux/uX) doesn't seem to be actively maintained and the current code
//! could use some big changes.
//!
//! So I did what any reasonable developer does and completely re-invented the wheel.
//!
//! Behold uX2, slightly better in almost every way.
//!
//! - More functionality, with optional support for `serde`.
//! - Better documentation.
//! - Better CI (e.g. automated changelog)
//!
//! I've already implemented some of the open issues from uX in this library e.g.
//! - <https://github.com/rust-ux/uX/issues/55>
//! - <https://github.com/rust-ux/uX/issues/54>
//! - <https://github.com/rust-ux/uX/issues/53>
//! - <https://github.com/rust-ux/uX/issues/17>
//!
//! Why didn't I just post a PR on uX?
//! 1. Review: The current PRs don't seem to be getting reviewed, I wasn't really confident a PR
//! which completely changes the entire library would be merged. 2. Control: If the maintainer/s of
//! uX are inactive there is nothing I can do, I cannot get PRs merged or fix issue, if I have
//! control I can do this.
//!
//! ## Features
//!
//! The `8`, `16`, `32`, `64` and `128` features enable support up to the types of `i8`/`u8`,
//! `i16`/`u16`, `i32`/`u32`, `i64`/`u64` and `i128`/`u128` respectively.
//!
//! The compile times increase exponentially, 3s, 7s, 30s, 3m and 46m respectively.
pub trait StdConversionTarget{
    type Target;
}
trait StdConversion:StdConversionTarget{
    fn to_std(&self) -> Self::Target;
    fn from_std(from: &Self::Target) -> Option<Self> where Self: Sized;
}
macro_rules! delegate_impls {
    ($ty:ty) => {
impl core::cmp::PartialOrd<Self> for $ty {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_std_const().partial_cmp(&other.to_std_const())
    }
    fn lt(&self, other: &Self) -> bool {
        self.to_std_const().lt(&other.to_std_const())
    }
    fn le(&self, other: &Self) -> bool {
        self.to_std_const().le(&other.to_std_const())
    }
    fn gt(&self, other: &Self) -> bool {
        self.to_std_const().gt(&other.to_std_const())
    }
    fn ge(&self, other: &Self) -> bool {
        self.to_std_const().ge(&other.to_std_const())
    }
}
impl core::cmp::Ord for $ty {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.to_std_const().cmp(&other.to_std_const())
    }
}
impl core::fmt::Debug for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.to_std_const(), f)
    }
}
impl core::fmt::Display for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.to_std_const(), f)
    }
}
impl core::fmt::Binary for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.to_std_const(), f)
    }
}
impl core::fmt::LowerHex for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.to_std_const(), f)
    }
}
impl core::fmt::UpperHex for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.to_std_const(), f)
    }
}
impl core::fmt::Octal for $ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.to_std_const(), f)
    }
}
impl crate::StdConversion for $ty {
    fn to_std(&self) -> Self::Target{
        self.to_std_const()
    }
    fn from_std(from: &Self::Target) -> Option<Self> {
        Self::from_std_const(from)
    }
}
impl core::convert::From<$ty> for <$ty as $crate::StdConversionTarget>::Target {
    fn from(value: $ty) -> Self{
        value.to_std_const()
    }
}
    };
}

ux3_macros::define_enum!(1, 2, 3, 4, 5, 6, 7);
// ux3_macros::define_enum!(12);

/// A mimic of [`std::num::TryFromIntError`] that can be constructed on stable.
#[derive(Debug, Eq, PartialEq)]
pub struct TryFromIntError;
impl core::fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed `TryFrom`.")
    }
}
#[cfg(not(feature = "nostd"))]
impl std::error::Error for TryFromIntError {}

/// A mimic of [`std::num::ParseIntError`] that can be constructed on stable.
#[derive(Debug, Eq, PartialEq)]
pub struct ParseIntError;
impl core::fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed `TryFrom`.")
    }
}
#[cfg(not(feature = "nostd"))]
impl std::error::Error for ParseIntError {}

/// https://doc.rust-lang.org/std/primitive.array.html#method.split_array_mut
fn array_split_array_mut<T, const N: usize, const M: usize>(
    array: &mut [T; N],
) -> (&mut [T; M], &mut [T]) {
    slice_split_array_mut::<_, M>(&mut array[..])
}

/// https://doc.rust-lang.org/std/primitive.array.html#method.rsplit_array_mut
fn array_rsplit_array_mut<T, const N: usize, const M: usize>(
    array: &mut [T; N],
) -> (&mut [T], &mut [T; M]) {
    slice_rsplit_array_mut::<_, M>(&mut array[..])
}

/// https://doc.rust-lang.org/std/primitive.slice.html#method.rsplit_array_mut
fn slice_rsplit_array_mut<T, const N: usize>(slice: &mut [T]) -> (&mut [T], &mut [T; N]) {
    assert!(N <= slice.len());
    let (a, b) = slice.split_at_mut(slice.len() - N);
    // SAFETY: b points to [T; N]? Yes it's [T] of length N (checked by split_at_mut)
    unsafe { (a, &mut *(b.as_mut_ptr() as *mut [T; N])) }
}

/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_array_mut
fn slice_split_array_mut<T, const N: usize>(slice: &mut [T]) -> (&mut [T; N], &mut [T]) {
    let (a, b) = slice.split_at_mut(N);
    // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at_mut)
    unsafe { (&mut *(a.as_mut_ptr() as *mut [T; N]), b) }
}
