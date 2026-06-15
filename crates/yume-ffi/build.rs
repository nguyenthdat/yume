/// Generate UniFFI scaffolding at build time.
/// The Kotlin bindings are generated separately via `just ffi-bindings`.
fn main() {
    uniffi::generate_scaffolding("src/yume.udl").unwrap();
}
