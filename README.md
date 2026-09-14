# Supertest for Rust

```sh
cargo add schematic-supertest
```

```rust
use schematic::{assume, supertest};

#[supertest]
fn integer_division_is_bounded(value: i32, divisor: i32) {
    assume(value >= 0 && divisor > 0);
    assert!(value / divisor <= value);
}
```

See the [Getting Started Documentation](https://docs.schematic.tech/pup).

## License

This library is available under either MIT or Apache-2.0, at your option.
