# Contributing

Thanks for your interest in improving `rust-playground`.

This is a personal playground for Rust, Python, and WebAssembly packaging
experiments. Small, focused fixes and documentation improvements are welcome.

## Communication

- Use English for issue titles, pull request titles, and pull request
  descriptions.
- Keep each issue or pull request focused on one topic.
- Do not include secrets, access tokens, private package names, or credentials in
  issues, pull requests, logs, or screenshots.
- For larger behavior changes, open an issue first so the scope can be agreed on
  before implementation.

## Development Setup

Install a current stable Rust toolchain with Cargo. Optional tools are only
needed when you touch their area:

- `uv` for Python package work under `python/`;
- `wasm-pack` for WebAssembly package work under `wasm/`;
- `lefthook` if you want Git hooks that mirror the core CI checks locally.

To install the hooks once:

```sh
lefthook install
```

## Local Checks

Run the checks that match your change before opening a pull request. For changes
to Rust code or shared workspace configuration, run:

```sh
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
cargo check
```

If `cargo-audit` is installed, also run:

```sh
cargo audit
```

For documentation-only changes, run a spelling check if you have one available
and make sure Markdown examples stay accurate.

## Issues

Use the issue templates when possible. Bug reports are most useful when they
include:

- a short summary;
- steps to reproduce;
- expected and actual behavior;
- operating system, Rust version, and package or crate name;
- relevant logs or error messages.

Feature requests should describe the use case and the smallest change that would
solve it.

## Pull Requests

Before opening a pull request:

- keep the diff limited to one topic;
- update tests or documentation when behavior changes;
- include the local checks you ran;
- explain any checks you could not run.

CI runs the full suite on pull requests. Passing local checks does not replace CI
but keeps review focused on the change itself.

## License

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository, as defined in the Apache-2.0 license, is dual
licensed under MIT or Apache-2.0 without additional terms or conditions.
