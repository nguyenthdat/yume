use camino::Utf8PathBuf;

fn main() {
    let udl = Utf8PathBuf::from("crates/yume-ffi/src/yume.udl");
    let out = Utf8PathBuf::from("android/app/src/main/java/com/yume/rust");
    std::fs::create_dir_all(&out).unwrap();
    uniffi::generate_bindings(
        &udl,
        None,
        uniffi::KotlinBindingGenerator,
        Some(&out),
        None,
        None,
        false,
    )
    .unwrap();
    println!("✅ Kotlin bindings → {}", out);
}
