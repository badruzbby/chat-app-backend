fn main() {
    // Memberitahu Rust bahwa 'ci' adalah flag konfigurasi yang valid
    println!("cargo:rustc-check-cfg=cfg(ci)");
    
    // Deteksi jika sedang dalam lingkungan CI
    if std::env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci");
    }
}
