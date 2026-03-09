fn main() {
    // Tell Cargo to rerun if translation files change
    println!("cargo:rerun-if-changed=locales");
}
