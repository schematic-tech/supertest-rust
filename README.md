# Supertest for Rust

The Cargo package is `schematic-supertest`, exposing the `schematic` library crate.
Until it is published, add a path dependency (adjust the path to your checkout):

```toml
[dependencies]
schematic-supertest = { path = "../supertest/supertest-rust" }
```

```rust
use schematic::{assume, supertest};

#[supertest]
fn integer_division_is_bounded(value: i32, divisor: i32) {
    assume(value >= 0 && divisor > 0);
    assert!(value / divisor <= value);
}
```

The attribute preserves the item without invoking it or registering a unit test.
A true assumption continues. A false assumption calls `std::process::exit(0)` and
ends the entire process successfully. Run one input per process. This deliberately
does not panic, cannot be intercepted by `catch_unwind`, and does not run stack
destructors. The old `AssumptionNotMet` panic payload is removed. Requires `std`.

These primitives do not generate inputs or prove properties; checker tooling
interprets the source independently of ordinary runtime execution.

```sh
cargo fmt --all -- --check
cargo test --locked --workspace
cargo doc --no-deps --workspace
```

The ignored `runtime_probe` test is intentionally invoked in a child process by
other tests, keeping process exit away from the parent harness.

Adapted from `schematic-tech/schematic-supertests` at
`b6ccba3b5e0d42ea2846d0476eeb9496afb4a7f4`.

## Distribution

The public crates are `schematic-supertest` and its internal dependency
`schematic-supertest-macros` on crates.io. Users add only the main crate:

```sh
cargo add schematic-supertest
```

Source imports remain `use schematic::{assume, supertest};`. The minimum supported
Rust version is 1.85. See [release automation and crates.io setup](RELEASING.md).

## License

This interoperability library is available under either [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option.
