# uX3: A better [uX](https://github.com/rust-ux/uX)/[ux2](https://github.com/JonathanWoollett-Light/ux2)

[![Crates.io](https://img.shields.io/crates/v/ux3)](https://crates.io/crates/ux4)
[![docs](https://img.shields.io/crates/v/ux3?color=yellow&label=docs)](https://docs.rs/ux3)

Please note that this readme is inherited from ux2, and may be largely incorrect.
My main point for creating ux3 is, because ux2 is cumbersome to use at times and that their types can theoretically hold more memory than is required.
ux3's types only use as much space as nessesary (a ux only (theoretically) uses x bits of storage. e.g. `Option<u7>` should only occupy 1 byte). This is achieved by utilizing enums for u1 to u7 and i1 to i7. 

#### Non-standard integer types like `u7`, `u9`, `u10`, `u63`, `i7`, `i9` etc.

When non-standard-width integers are required in an application, the norm is to use a larger container and make sure the value is within range after manipulation. uX2 aims to take care of this once and for all by providing `u1`-`u127` and `i1`-`i127` types (depending on the enabled features) that offer safe arithmetic operations.

`<core::primitive::i32 as core::ops::Add<core::primitive::i32>>::add` can panic in `Debug` or overflow in `Release`, `<ux2::i32 as core::ops::Add<ux2::i32>>::add` cannot panic or overflow in `Debug` or `Release`, this is because it returns `ux2::i33`. This is applied for all operations and combinations of types in `ux2`. This allows for more thorough compile time type checking.

```rust
use rand::Rng;
let a = ux2::i4::try_from(3i8).unwrap();
let b = ux2::i8::from(rand::thread_rng().gen::<core::primitive::i8>());
let c: ux2::i9 = a + b;
let d: ux2::i4 = c % a;
let e: core::primitive::i8 = core::primitive::i8::from(d);
```

uX2 types take up as much space as the smallest integer type that can contain them.

## Features

The `8`, `16`, `32`, `64` and `128` features enable support up to the types of `i8`/`u8`, `i16`/`u16`, `i32`/`u32`, `i64`/`u64` and `i128`/`u128` respectively.

The compile times increase exponentially, 3s, 7s, 30s, 3m and 46m respectively.

## Why does this exist? Why use this over `ux`?

I noticed [uX](https://github.com/rust-ux/uX) doesn't seem to be actively maintained and the current code
could use some big changes.

So I did what any reasonable developer does and completely re-invented the wheel.

Behold uX2, slightly better in almost every way.

- More functionality, with optional support for `serde`.
- Better documentation.
- Better CI (e.g. automated changelog)

I've already implemented some of the open issues from uX in this library e.g.
- https://github.com/rust-ux/uX/issues/55
- https://github.com/rust-ux/uX/issues/54
- https://github.com/rust-ux/uX/issues/53
- https://github.com/rust-ux/uX/issues/17

Why didn't I just post a PR on uX?
1. Review: The current PRs don't seem to be getting reviewed, I wasn't really confident a PR which completely changes the entire library would be merged.
2. Control: If the maintainer/s of uX are inactive there is nothing I can do, I cannot get PRs merged or fix issue, if I have control I can do this.
