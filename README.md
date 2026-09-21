# Supertest for Rust

```sh
cargo add schematic-supertest
```

From the [text-tools example](examples/text-tools):

```rust
use schematic::supertest;
use text_tools::collapse_spaces;

#[supertest]
pub fn collapsing_spaces_again_changes_nothing(text: String) {
    let once = collapse_spaces(&text);
    let twice = collapse_spaces(&once);

    assert_eq!(twice, once);
}
```

See the [Getting Started Documentation](https://docs.schematic.tech/super).

## Example

Try [text-tools](https://github.com/schematic-tech/supertest-rust/tree/main/examples/text-tools). It includes a supertest
that finds a space-normalization bug.

## License

This library is available under either MIT or Apache-2.0, at your option.
