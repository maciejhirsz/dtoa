ftoa
====

[![Latest Version](https://img.shields.io/crates/v/ftoa.svg)](https://crates.io/crates/ftoa)

This is a fork of the [`dtoa` crate](https://github.com/dtolnay/dtoa) which writes integer value floats without trailing `.0`.

This crate provides fast functions for printing floating-point primitives to an
[`io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html). The
implementation is a straightforward Rust port of [Milo
Yip](https://github.com/miloyip)'s C++ implementation
[ftoa.h](https://github.com/miloyip/rapidjson/blob/master/include/rapidjson/internal/ftoa.h).
The original C++ code of each function is included in comments.

## Performance

![performance](https://raw.githubusercontent.com/dtolnay/ftoa/master/performance.png)

## Functions

```rust
extern crate ftoa;

let mut buf = Vec::new();
ftoa::write(&mut buf, 2.71828f64).unwrap();
```

The function signature is:

```rust
fn write<W: io::Write, V: ftoa::Floating>(writer: &mut W, value: V) -> io::Result<()>
```

where `ftoa::Floating` is implemented for `f32` and `f64`.

## Dependency

ftoa is available on [crates.io](https://crates.io/crates/ftoa). Use the
following in `Cargo.toml`:

```toml
[dependencies]
ftoa = "0.1"
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in ftoa by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
