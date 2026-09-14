//! Source-level authoring primitives for Schematic Supertests.

pub use schematic_supertest_macros::supertest;

/// Declare that the current Supertest only makes a claim when `condition` is true.
///
/// Schematic tooling interprets this call directly. Ordinary Rust execution
/// exits the current process with status zero if the assumption is false.
/// Run one input per process. This does not unwind or run stack destructors.
pub fn assume(condition: bool) {
    if !condition {
        std::process::exit(0);
    }
}
