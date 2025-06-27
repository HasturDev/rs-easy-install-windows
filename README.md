# 🦀 Rust GNU/MSYS Installation Helper

> **Automated installer for Rust with GNU/MSYS toolchain on Windows**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Windows](https://img.shields.io/badge/platform-windows-blue.svg)](https://www.microsoft.com/windows)
[![License](https://img.shields.io/badge/license-MIT%20)](LICENSE)

A comprehensive, automated solution for installing and configuring Rust with the GNU/MSYS toolchain on Windows systems. This program handles everything from downloading MSYS2 to configuring your development environment, making it easy to get started with Rust development using Unix-compatible tools.

## 🎯 Why Use This?

### **GNU Toolchain Benefits:**
- **🔧 Better Unix Compatibility**: Works seamlessly with Unix-like tools and libraries
- **📦 Open Source**: Completely free and open-source toolchain (GCC-based)
- **🔄 Cross-Platform Consistency**: Similar behavior to Linux/macOS builds
- **📚 Better Library Support**: Some C/C++ libraries work better with GCC than MSVC
- **🛠️ Development Tools**: Access to full GNU development ecosystem

### **Why Not MSVC?**
While MSVC is Microsoft's native toolchain and works great for Windows-specific development, the GNU toolchain provides:
- Better compatibility with cross-platform projects
- Easier integration with Unix-based build systems
- Support for libraries that expect GCC-style compilation

## ✨ Features

- **🚀 Fully Automated**: Zero manual intervention required
- **📥 Auto-Download**: Automatically downloads and installs MSYS2 if not present
- **⚙️ Complete Configuration**: Sets up Rust, GNU toolchain, and environment
- **✅ Verification**: Tests installation with sample compilation
- **🔧 Smart Error Handling**: Handles common installation issues gracefully
- **📋 Detailed Logging**: Provides clear feedback throughout the process
- **🔄 Idempotent**: Safe to run multiple times
- **🛡️ Safe**: Non-destructive - won't break existing installations

## 🏗️ What This Program Does

### **Automatic Installation Process:**

1. **🔍 System Check**: Detects existing Rust and MSYS2 installations
2. **📦 MSYS2 Setup**: Downloads and installs MSYS2 if not present
3. **⚙️ Package Installation**: Installs GNU toolchain components:
   - `mingw-w64-x86_64-toolchain` (GCC, binutils, etc.)
   - `mingw-w64-x86_64-cmake` (Build system)
   - `mingw-w64-x86_64-pkgconf` (Package configuration)
   - `mingw-w64-x86_64-openssl` (Crypto library)
   - `mingw-w64-x86_64-make` (Build tools)
4. **🦀 Rust Configuration**: Adds `x86_64-pc-windows-gnu` target
5. **📁 Environment Setup**: Creates `.cargo/config.toml` with GNU settings
6. **✅ Verification**: Compiles and runs test program

## 🚀 Quick Start

### **Prerequisites**
- Windows 10/11 (64-bit)
- Internet connection
- PowerShell (included with Windows)
- ~2GB free disk space

### **Installation**

1. **Clone or download this repository:**
   ```bash
   git clone https://github.com/yourusername/rust-gnu-msys-installer.git
   cd rust-gnu-msys-installer
   ```

2. **Build and run the installer:**
   ```bash
   cargo build --release
   cargo run
   ```

3. **Follow the automated process** - the program will guide you through everything!

### **Alternative: Direct Download**
If you don't have Rust installed yet:
1. Download the pre-built executable from [Releases](../../releases)
2. Run `install-rust-gnu.exe`

## 📋 Usage

### **Basic Usage**
```bash
# Build and run
cargo run

# Or run the release binary
./target/release/install-rust-gnu.exe
```

### **What You'll See**
```
🦀 Rust GNU/MSYS Installation Helper for Windows
================================================

🔍 Checking for existing installations...
✅ Found MSYS2 installation at: C:\msys64\usr\bin\bash.exe

🔧 Installing GNU Toolchain
---------------------------
Installing Core toolchain: pacman -S --noconfirm mingw-w64-x86_64-toolchain
✅ Core toolchain - installed successfully

🦀 Installing Rust with GNU Target
----------------------------------
✅ rustup found. Adding GNU target...
✅ x86_64-pc-windows-gnu target added successfully!

🧪 Testing compilation...
✅ Test program executed successfully:
   Hello from Rust with GNU toolchain!
   ✅ Successfully using GNU environment!

✅ Installation process completed successfully!
```

## ⚙️ Configuration

### **Generated Configuration**

The program creates `.cargo/config.toml` with optimal GNU settings:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[build]
target = "x86_64-pc-windows-gnu"

[env]
CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc"
CXX_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-g++"
```

### **Environment Variables**

Add these to your PATH:
```
C:\msys64\mingw64\bin
C:\msys64\usr\bin
```

### **Switching Between Toolchains**

After installation, you can switch between GNU and MSVC:

```bash
# Use GNU toolchain (default after this installer)
rustup override set stable-x86_64-pc-windows-gnu

# Switch back to MSVC
rustup override set stable-x86_64-pc-windows-msvc

# Check current target
rustup show active-toolchain
```

## 🛠️ Development

### **Building New Projects**

After installation, create new Rust projects that use GNU by default:

```bash
# Create new project
cargo new my_gnu_project
cd my_gnu_project

# Build with GNU toolchain (automatic if configured)
cargo build

# Explicitly specify GNU target
cargo build --target x86_64-pc-windows-gnu
```

### **Cross-Compilation**

Your system will now support both targets:

```bash
# Build for GNU (Unix-compatible)
cargo build --target x86_64-pc-windows-gnu

# Build for MSVC (Windows-native)  
cargo build --target x86_64-pc-windows-msvc
```

## 🐛 Troubleshooting

### **Common Issues**

#### **PowerShell Execution Policy**
```
Error: PowerShell execution policy blocks downloads
```
**Solution**: Run as administrator or enable PowerShell scripts:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

#### **Permission Denied During Installation**
```
Error: Access denied when installing MSYS2
```
**Solution**: Run the program as administrator

#### **Package Installation Failures**
```
Warning: pkg-config - skipped (dependency issue)
```
**Solution**: This is usually harmless. Core functionality will still work.

#### **Path Issues**
```
Warning: Compiled successfully but couldn't run
```
**Solution**: Add MSYS2 paths to your system PATH:
- `C:\msys64\mingw64\bin`
- `C:\msys64\usr\bin`

### **Manual Recovery**

If something goes wrong, you can:

1. **Reset Rust toolchain:**
   ```bash
   rustup toolchain uninstall stable-x86_64-pc-windows-gnu
   rustup toolchain install stable-x86_64-pc-windows-gnu
   ```

2. **Reinstall MSYS2**: Delete `C:\msys64` and run the installer again

3. **Clean cargo config**: Delete `.cargo/config.toml` to reset settings

### **Verification Commands**

Check your installation:

```bash
# Check Rust targets
rustup target list --installed

# Check current toolchain
rustup show active-toolchain

# Test GNU compilation
echo 'fn main() { println!("Hello GNU!"); }' > test.rs
rustc test.rs --target x86_64-pc-windows-gnu
./test.exe
```

## 📊 Comparison: GNU vs MSVC

| Feature | GNU/MSYS | MSVC |
|---------|----------|------|
| **License** | Open Source | Proprietary |
| **Unix Compatibility** | ✅ Excellent | ❌ Poor |
| **Windows Integration** | ⚠️ Good | ✅ Native |
| **C++ Standard Library** | libstdc++ | MSVC STL |
| **Debugging** | GDB | Visual Studio |
| **Binary Size** | Smaller | Larger |
| **Compile Speed** | Fast | Fast |
| **Cross-Platform** | ✅ Consistent | ❌ Windows-only |

## 🏗️ Project Structure

```
rust-gnu-msys-installer/
├── src/
│   └── main.rs              # Main installer program
├── Cargo.toml               # Project configuration
├── README.md               # This file
├── LICENSE                 # License file
└── .gitignore             # Git ignore rules
```

## 🧪 Testing

Run the test suite:

```bash
# Run unit tests
cargo test

# Test in release mode
cargo test --release

# Test specific functions
cargo test test_windows_check
```

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run tests**: `cargo test`
6. **Commit changes**: `git commit -am 'Add amazing feature'`
7. **Push to branch**: `git push origin feature/amazing-feature`
8. **Create Pull Request**

### **Development Guidelines**

- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add tests for new features
- Update documentation as needed
- Follow Rust naming conventions

## 📝 License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- **MSYS2 Project**: For providing the excellent Unix-compatible environment
- **Rust Team**: For creating an amazing programming language
- **MinGW-w64**: For the GNU toolchain on Windows
- **Community**: For feedback and contributions

## 🔗 Related Links

- [Rust Official Website](https://www.rust-lang.org/)
- [MSYS2 Project](https://www.msys2.org/)
- [MinGW-w64](https://www.mingw-w64.org/)
- [rustup Documentation](https://rust-lang.github.io/rustup/)

## 📞 Support

- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)
- **Email**: hasturdev@gmail.com

---

**Made with ❤️ for the Rust community**

*Happy coding with Rust and GNU tools! 🦀🔧*
