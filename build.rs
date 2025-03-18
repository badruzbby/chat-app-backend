fn main() {
    // Deteksi jika sedang dalam lingkungan CI
    if std::env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci");
    }
} 