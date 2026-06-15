/// Generate UniFFI scaffolding at build time.
fn main() {
    uniffi::generate_scaffolding("src/yume.udl").unwrap();
}
