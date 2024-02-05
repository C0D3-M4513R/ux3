# uX3: A better [uX](https://github.com/rust-ux/uX)/[ux2](https://github.com/JonathanWoollett-Light/ux2)

[![Crates.io](https://img.shields.io/crates/v/ux3)](https://crates.io/crates/ux4)
[![docs](https://img.shields.io/crates/v/ux3?color=yellow&label=docs)](https://docs.rs/ux3)

Please note that this readme is inherited from ux2, and may be largely incorrect.
My main point for creating ux3 is, because ux2 is cumbersome to use at times and that their types can theoretically hold more memory than is required.
ux3's types only use as much space as nessesary (a ux only (theoretically) uses x bits of storage. e.g. `Option<u7>` should only occupy 1 byte). This is achieved by utilizing enums for u1 to u7 and i1 to i7. 

#### Non-standard integer types like `u7`, `u9`, `u10`, `u63`, `i7`, `i9` etc.
# Traits that should be implemented

## Comparisons (std::cmp)
* [x]  `PartialOrd<Self>`
* [x]  `Ord`
* [x]  `PartialEq<Self>`
* [x]  `Eq`

## Other
* [x]  `std::hash::Hash`
* [x]  `std::default::Default`

## Format (std::fmt)
* [x]  `Binary`
* [x]  `Debug`
* [x]  `LowerHex`
* [x]  `UpperHex`
* [x]  `Display`
* [x]  `Octal`


## Conversion (std::convert)
* [ ]  `TryFrom<T>`
* [ ]  `From<T>`
* [ ]  `FromStr`

## Operations (std::ops)
* [ ]  `Add<Self>`
* [ ]  `Add<&Self>`
* [ ]  `Add<Self> for &Self`
* [ ]  `Add<&Self>for &Self`
* [ ]  `AddAssign<Self>`
* [ ]  `AddAssign<&Self>`
* [ ]  `BitAnd<Self>`
* [ ]  `BitAnd<&Self>`
* [ ]  `BitAnd<Self> for &Self`
* [ ]  `BitAnd<&Self> for &Self`
* [ ]  `BitAndAssign<Self>`
* [ ]  `BitAndAssign<&Self>`
* [ ]  `BitOr<Self>`
* [ ]  `BitOr<&Self>`
* [ ]  `BitOr<Self> for &Self`
* [ ]  `BitOr<&Self> for &Self`
* [ ]  `BitOrAssign<Self>`
* [ ]  `BitOrAssign<&Self>`
* [ ]  `BitXor<Self>`
* [ ]  `BitXor<&Self>`
* [ ]  `BitXor<Self> for &Self`
* [ ]  `BitXor<&Self> for &Self`
* [ ]  `BitXorAssign<Self>`
* [ ]  `BitXorAssign<&Self>`
* [ ]  `Div<Self>`
* [ ]  `Div<&Self>`
* [ ]  `Div<Self> for &Self`
* [ ]  `Div<&Self> for &Self`
* [ ]  `DivAssign<Self>`
* [ ]  `DivAssign<&Self>`
* [ ]  `Sum<Self>`
* [ ]  `Sum<&Self>`
* [ ]  `Mul<Self>`
* [ ]  `Mul<&Self>`
* [ ]  `Mul<Self> for &Self`
* [ ]  `Mul<&Self> for &Self`
* [ ]  `MulAssign<Self>`
* [ ]  `MulAssign<&Self>`
* [ ]  `Not`
* [ ]  `Not for &Self`
* [ ]  `Product<Self>`
* [ ]  `Product<&Self>`
* [ ]  `Rem<Self>`
* [ ]  `Rem<&Self>`
* [ ]  `Rem<Self> for &Self`
* [ ]  `Rem<&Self> for &Self`
* [ ]  `RemAssign<Self>`
* [ ]  `RemAssign<&Self>`
* [ ]  `Shl<T>` (for all possible numeric `T` and `&T`)
* [ ]  `Shl<&T>` (for all possible numeric `T` and `&T`)
* [ ]  `Shr<T>` (for all possible numeric `T` and `&T`)
* [ ]  `Shr<&T>` (for all possible numeric `T`and `&T`)
* [ ]  `ShlAssign<T>` (for all possible numeric `T`and `&T`)
* [ ]  `ShlAssign<&T>` (for all possible numeric `T`and `&T`)
* [ ]  `ShrAssign<T>` (for all possible numeric `T`and `&T`)
* [ ]  `ShrAssign<&T>` (for all possible numeric `T`and `&T`)
* [ ]  `Sub<Self>`
* [ ]  `Sub<&Self>`
* [ ]  `Sub<Self> for &Self`
* [ ]  `Sub<&Self> for &Self`
* [ ]  `SubAssign<Self>`
* [ ]  `SubAssign<&Self>`