fn main() {
    println!("Hello from Rust with MSVC toolchain!");
    println!("Target: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
    
    #[cfg(target_env = "msvc")]
    println!("✅ Successfully using MSVC environment!");
    
    #[cfg(not(target_env = "msvc"))]
    println!("⚠️  Not using MSVC environment");
}