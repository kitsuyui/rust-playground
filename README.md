# rust-playground

[![Crates.io](https://img.shields.io/crates/d/kitsuyui-rust-playground)](https://crates.io/crates/kitsuyui-rust-playground)
[![Crates.io](https://img.shields.io/crates/d/kitsuyui-rust-playground-lib)](https://crates.io/crates/kitsuyui-rust-playground-lib)
[![crates.io-LICENSE](https://img.shields.io/crates/l/kitsuyui-rust-playground)](https://github.com/kitsuyui/rust-playground#license)
[![codecov](https://codecov.io/gh/kitsuyui/rust-playground/branch/main/graph/badge.svg?token=6WIF2MIHWQ)](https://codecov.io/gh/kitsuyui/rust-playground)
[![docs.rs](https://img.shields.io/docsrs/kitsuyui-rust-playground-lib)](https://docs.rs/kitsuyui-rust-playground-lib/0.1.5/kitsuyui_rust_playground_lib/)

## en: What is this?

This is my personal Rust playground.
I'm going to try to speed up CI and publish binaries on crates.io.

## ja: これは何？

私の個人的な Rust の実験場です
とくに CI 周りの高速化やバイナリの配布 crates.io での公開を試すつもりです。

## TODO

- [x] Separation by workspace
  - [x] bin crate
  - [x] lib crate
- [x] Build for multiple architectures
  - executable binaries
    - [x] aarch64-apple-darwin
    - [x] aarch64-unknown-linux-gnu
    - [x] aarch64-unknown-linux-musl
    - [x] arm-unknown-linux-gnueabihf
    - [x] arm-unknown-linux-musleabihf
    - [x] armv7-unknown-linux-gnueabihf
    - [x] armv7-unknown-linux-musleabihf
    - [x] x86_64-apple-darwin
    - [x] x86_64-pc-windows-gnu
    - [x] x86_64-unknown-linux-gnu
    - [x] x86_64-unknown-linux-musl
  - shared libraries
    - [x] aarch64-apple-darwin
    - [x] aarch64-unknown-linux-gnu
    - [x] arm-unknown-linux-gnueabihf
    - [x] armv7-unknown-linux-gnueabihf
    - [x] x86_64-apple-darwin
    - [x] x86_64-pc-windows-gnu
    - [x] x86_64-unknown-linux-gnu
- [x] Publish lib crate to crates.io
- [x] WASM
- [x] FFI
  - [x] Node.js binding
  - [x] Python binding

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
