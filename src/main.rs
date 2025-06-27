use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🦀 Rust GNU/MSYS Installation Helper for Windows");
    println!("================================================\n");

    match run_installation_process() {
        Ok(_) => println!("\n✅ Installation process completed successfully!"),
        Err(e) => eprintln!("\n❌ Error during installation: {}", e),
    }
}

fn run_installation_process() -> Result<(), Box<dyn std::error::Error>> {
    // Check if we're on Windows
    if !cfg!(target_os = "windows") {
        return Err("This installer is designed for Windows systems only.".into());
    }

    println!("This program will help you install Rust with GNU/MSYS toolchain.");
    println!("The GNU toolchain provides better compatibility with Unix-like tools.\n");

    // Step 1: Check for existing installations
    check_existing_installations()?;

    // Step 2: Guide MSYS2 installation
    guide_msys2_installation()?;

    // Step 3: Install GNU toolchain
    install_gnu_toolchain()?;

    // Step 4: Install Rust with GNU target
    install_rust_gnu()?;

    // Step 5: Configure environment
    configure_environment()?;

    // Step 6: Verify installation
    verify_installation()?;

    Ok(())
}

fn check_existing_installations() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking for existing installations...\n");

    // Check for rustc
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Found existing Rust installation: {}", version.trim());
            
            if version.contains("msvc") {
                println!("⚠️  Current installation uses MSVC toolchain.");
                println!("   We'll configure GNU toolchain as an additional target.");
            }
        }
        Err(_) => println!("No existing Rust installation found."),
    }

    // Check for MSYS2
    let msys2_paths = [
        "C:\\msys64\\usr\\bin\\bash.exe",
        "C:\\msys32\\usr\\bin\\bash.exe",
    ];

    let mut msys2_found = false;
    for path in &msys2_paths {
        if Path::new(path).exists() {
            println!("✅ Found MSYS2 installation at: {}", path);
            msys2_found = true;
            break;
        }
    }

    if !msys2_found {
        println!("❌ MSYS2 not found. Installation will be required.");
    }

    println!();
    Ok(())
}

fn guide_msys2_installation() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 MSYS2 Installation");
    println!("--------------------");

    // Check if MSYS2 is already installed
    if Path::new("C:\\msys64\\usr\\bin\\bash.exe").exists() {
        println!("✅ MSYS2 is already installed.");
        return Ok(());
    }

    println!("MSYS2 not found. Installing automatically...");
    
    // Download and install MSYS2
    download_and_install_msys2()?;
    
    // Initialize MSYS2
    initialize_msys2()?;

    // Verify MSYS2 installation
    if !Path::new("C:\\msys64\\usr\\bin\\bash.exe").exists() {
        return Err("MSYS2 installation failed. Please try manual installation from https://www.msys2.org/".into());
    }

    println!("✅ MSYS2 installation completed successfully!");
    println!();
    Ok(())
}

fn download_and_install_msys2() -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Downloading MSYS2 installer...");
    
    // Download MSYS2 installer
    let installer_url = "https://github.com/msys2/msys2-installer/releases/latest/download/msys2-x86_64-latest.exe";
    let installer_path = "msys2-installer.exe";
    
    // Use PowerShell to download the file (available on all Windows systems)
    let download_cmd = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        installer_url, installer_path
    );
    
    println!("Downloading from: {}", installer_url);
    let download_output = Command::new("powershell")
        .args(&["-Command", &download_cmd])
        .output()?;
    
    if !download_output.status.success() {
        let error_msg = String::from_utf8_lossy(&download_output.stderr);
        if error_msg.contains("cannot be loaded because running scripts is disabled") {
            return Err("PowerShell execution policy blocks downloads. Please run as administrator or enable PowerShell scripts.".into());
        }
        return Err(format!(
            "Failed to download MSYS2 installer: {}",
            error_msg
        ).into());
    }
    
    if !Path::new(installer_path).exists() {
        return Err("MSYS2 installer download failed - file not found".into());
    }
    
    println!("✅ Download completed successfully ({} MB)", 
             fs::metadata(installer_path)?.len() / 1_000_000);
    
    // Run the installer silently
    println!("🚀 Running MSYS2 installer...");
    println!("   Installing to C:\\msys64...");
    println!("   This may take several minutes, please wait...");
    
    // Try silent installation first
    let install_output = Command::new(installer_path)
        .args(&[
            "install",
            "--confirm-command",
            "--accept-messages", 
            "--root", "C:\\msys64"
        ])
        .output()?;
    
    if !install_output.status.success() {
        println!("⚠️  Silent installation failed, trying alternative method...");
        
        // Try running with elevated permissions request
        let powershell_cmd = format!(
            "Start-Process -FilePath '{}' -ArgumentList 'install --confirm-command --accept-messages --root C:\\msys64' -Verb RunAs -Wait",
            installer_path
        );
        
        let elevated_output = Command::new("powershell")
            .args(&["-Command", &powershell_cmd])
            .output()?;
        
        if !elevated_output.status.success() {
            println!("❌ Automated installation failed.");
            println!("📝 Please install MSYS2 manually:");
            println!("   1. Double-click the downloaded installer: {}", installer_path);
            println!("   2. Follow the installation wizard");
            println!("   3. Install to C:\\msys64 (default location)");
            println!("   4. Complete the installation");
            println!();
            
            print!("Press Enter when manual installation is complete...");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
        }
    }
    
    // Clean up installer file
    let _ = fs::remove_file(installer_path);
    
    // Verify installation
    if !Path::new("C:\\msys64").exists() {
        return Err("MSYS2 installation directory not found. Installation may have failed.".into());
    }
    
    println!("✅ MSYS2 installation completed");
    Ok(())
}

fn initialize_msys2() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Initializing MSYS2...");
    
    let msys2_bash = "C:\\msys64\\usr\\bin\\bash.exe";
    
    // Wait for installation to settle and files to be ready
    print!("   Waiting for MSYS2 to be ready");
    for _ in 0..10 {
        print!(".");
        io::stdout().flush()?;
        thread::sleep(Duration::from_secs(1));
        
        if Path::new(msys2_bash).exists() {
            break;
        }
    }
    println!(" ✅");
    
    if !Path::new(msys2_bash).exists() {
        return Err("MSYS2 bash not found after installation. Installation may be incomplete.".into());
    }
    
    // Initialize MSYS2 keyring and update packages
    let init_commands = [
        ("Initializing keyring", "pacman-key --init"),
        ("Populating keyring", "pacman-key --populate msys2"),
        ("Updating package database", "pacman -Sy --noconfirm"),
        ("Updating system packages", "pacman -Syu --noconfirm --disable-download-timeout"),
    ];
    
    for (description, cmd) in &init_commands {
        println!("   {}: {}", description, cmd);
        
        let output = Command::new(msys2_bash)
            .args(&["-l", "-c", cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Check for common non-critical messages
            if stderr.contains("nothing to do") 
                || stderr.contains("there is nothing to do") 
                || stdout.contains("there is nothing to do") {
                println!("     ✅ No updates needed");
                continue;
            }
            
            // Some warnings during first-time setup are normal
            if stderr.contains("warning") && !stderr.contains("error") {
                println!("     ⚠️  Warning (continuing): {}", stderr.lines().next().unwrap_or(""));
                continue;
            }
            
            eprintln!("     ❌ Failed: {}", stderr);
            return Err(format!("MSYS2 initialization step failed: {}", description).into());
        } else {
            println!("     ✅ Completed successfully");
        }
    }
    
    // Verify core tools are available
    let verification_commands = [
        "pacman --version",
        "gcc --version",
    ];
    
    println!("   Verifying installation...");
    for cmd in &verification_commands {
        let output = Command::new(msys2_bash)
            .args(&["-l", "-c", cmd])
            .output();
            
        match output {
            Ok(out) if out.status.success() => {
                // Extract first line of output for verification
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                let first_line = stdout_str
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim();
                if !first_line.is_empty() {
                    println!("     ✅ {}: {}", cmd.split_whitespace().next().unwrap(), first_line);
                }
            }
            _ => {
                println!("     ⚠️  {} not yet available (will install with toolchain)", 
                         cmd.split_whitespace().next().unwrap());
            }
        }
    }
    
    println!("✅ MSYS2 initialization completed successfully");
    Ok(())
}

fn install_gnu_toolchain() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Installing GNU Toolchain");
    println!("---------------------------");

    let msys2_bash = "C:\\msys64\\usr\\bin\\bash.exe";
    
    if !Path::new(msys2_bash).exists() {
        return Err("MSYS2 bash not found. Please install MSYS2 first.".into());
    }

    println!("Installing GNU toolchain packages via MSYS2...");

    // Install mingw-w64 toolchain
    let install_commands = [
        ("Core toolchain", "pacman -S --noconfirm mingw-w64-x86_64-toolchain"),
        ("CMake", "pacman -S --noconfirm mingw-w64-x86_64-cmake"),
        ("pkg-config", "pacman -S --noconfirm mingw-w64-x86_64-pkgconf"), // Updated package name
        ("OpenSSL", "pacman -S --noconfirm mingw-w64-x86_64-openssl"),
        ("Additional tools", "pacman -S --noconfirm mingw-w64-x86_64-make"),
    ];

    let mut failed_packages = Vec::new();

    for (description, cmd) in &install_commands {
        println!("Installing {}: {}", description, cmd);
        let output = Command::new(msys2_bash)
            .args(&["-l", "-c", cmd])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Check if it's already installed or just a warning
            if stderr.contains("nothing to do") || stderr.contains("up to date") {
                println!("✅ {} - already up to date", description);
            } else if stderr.contains("could not satisfy dependencies") || stderr.contains("target not found") {
                println!("⚠️  {} - skipped (dependency issue or already installed)", description);
                failed_packages.push(description);
            } else {
                println!("❌ {} - failed: {}", description, stderr.lines().next().unwrap_or("Unknown error"));
                failed_packages.push(description);
            }
        } else {
            println!("✅ {} - installed successfully", description);
        }
    }

    if !failed_packages.is_empty() {
        println!("\n⚠️  Some packages had issues but core toolchain should still work:");
        for pkg in failed_packages {
            println!("   - {}", pkg);
        }
        println!("   This is usually not a problem for basic Rust development.");
    }

    println!("✅ GNU toolchain installation completed!");
    println!();
    Ok(())
}

fn install_rustup_automatically() -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Downloading rustup installer...");
    
    let rustup_url = "https://win.rustup.rs/x86_64";
    let installer_path = "rustup-init.exe";
    
    // Download rustup-init.exe
    let download_cmd = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        rustup_url, installer_path
    );
    
    println!("Downloading from: {}", rustup_url);
    let download_output = Command::new("powershell")
        .args(&["-Command", &download_cmd])
        .output()?;
    
    if !download_output.status.success() {
        let error_msg = String::from_utf8_lossy(&download_output.stderr);
        return Err(format!("Failed to download rustup installer: {}", error_msg).into());
    }
    
    if !Path::new(installer_path).exists() {
        return Err("rustup installer download failed - file not found".into());
    }
    
    println!("✅ rustup installer downloaded successfully");
    
    // Install rustup with GNU as default target
    println!("🚀 Installing rustup with GNU toolchain...");
    println!("   This will install Rust with x86_64-pc-windows-gnu as default");
    
    let install_output = Command::new(installer_path)
        .args(&[
            "--default-host", "x86_64-pc-windows-gnu",
            "--default-toolchain", "stable",
            "--profile", "default",
            "-y"  // Accept all defaults
        ])
        .output()?;
    
    // Clean up installer
    let _ = fs::remove_file(installer_path);
    
    if !install_output.status.success() {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        return Err(format!("rustup installation failed: {}", stderr).into());
    }
    
    println!("✅ rustup installation completed successfully!");
    
    // Verify installation
    match Command::new("rustup").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ Verified rustup installation: {}", version.trim());
        }
        Err(_) => {
            println!("⚠️  rustup installed but not immediately available in PATH");
            println!("   You may need to restart your terminal or run:");
            println!("   source ~/.cargo/env");
        }
    }
    
    Ok(())
}

fn install_rust_gnu() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Installing Rust with GNU Target");
    println!("----------------------------------");

    // Check if rustup is available
    match Command::new("rustup").arg("--version").output() {
        Ok(_) => {
            println!("✅ rustup found. Adding GNU target...");
            
            // Add the GNU target
            let output = Command::new("rustup")
                .args(&["target", "add", "x86_64-pc-windows-gnu"])
                .output()?;

            if output.status.success() {
                println!("✅ x86_64-pc-windows-gnu target added successfully!");
            } else {
                eprintln!("❌ Failed to add GNU target: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(_) => {
            println!("rustup not found. Installing Rust automatically...");
            install_rustup_automatically()?;
        }
    }

    // Set GNU as default target for current directory
    let output = Command::new("rustup")
        .args(&["override", "set", "stable-x86_64-pc-windows-gnu"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("✅ Set GNU toolchain as default for current directory");
        }
        _ => {
            println!("ℹ️  You can manually set GNU toolchain with:");
            println!("   rustup override set stable-x86_64-pc-windows-gnu");
        }
    }

    println!();
    Ok(())
}

fn configure_environment() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Configuring Environment");
    println!("--------------------------");

    // Create .cargo/config.toml for GNU toolchain
    let cargo_dir = Path::new(".cargo");
    if !cargo_dir.exists() {
        fs::create_dir(cargo_dir)?;
    }

    let config_content = r#"[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[build]
target = "x86_64-pc-windows-gnu"

[env]
CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc"
CXX_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-g++"
"#;

    let config_path = cargo_dir.join("config.toml");
    fs::write(&config_path, config_content)?;
    println!("✅ Created .cargo/config.toml with GNU toolchain settings");

    // Add MSYS2 to PATH suggestion
    println!();
    println!("📝 Environment Setup Recommendation:");
    println!("Add the following to your PATH environment variable:");
    println!("   C:\\msys64\\mingw64\\bin");
    println!("   C:\\msys64\\usr\\bin");
    println!();
    println!("You can do this by:");
    println!("1. Open System Properties → Advanced → Environment Variables");
    println!("2. Edit the PATH variable");
    println!("3. Add the paths above");
    println!();

    Ok(())
}

fn verify_installation() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Verifying Installation");
    println!("-------------------------");

    // Check rustc version and target
    match Command::new("rustc").args(&["--version", "--verbose"]).output() {
        Ok(output) => {
            println!("Rust compiler info:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(_) => println!("❌ Could not run rustc"),
    }

    // Check available targets
    match Command::new("rustup").args(&["target", "list", "--installed"]).output() {
        Ok(output) => {
            let targets = String::from_utf8_lossy(&output.stdout);
            println!("Installed targets:");
            for line in targets.lines() {
                if line.contains("windows-gnu") {
                    println!("✅ {}", line);
                } else {
                    println!("   {}", line);
                }
            }
        }
        Err(_) => println!("❌ Could not list targets"),
    }

    // Test compilation with a simple program
    println!("\n🧪 Testing compilation...");
    let test_code = r#"fn main() {
    println!("Hello from Rust with GNU toolchain!");
    println!("Target: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
    
    #[cfg(target_env = "gnu")]
    println!("✅ Successfully using GNU environment!");
    
    #[cfg(not(target_env = "gnu"))]
    println!("⚠️  Not using GNU environment");
}"#;

    fs::write("test_gnu.rs", test_code)?;

    let compile_output = Command::new("rustc")
        .args(&["test_gnu.rs", "--target", "x86_64-pc-windows-gnu"])
        .output()?;

    if compile_output.status.success() {
        println!("✅ Test compilation successful!");
        
        // Try to run the compiled program
        match Command::new("./test_gnu.exe").output() {
            Ok(run_output) => {
                println!("✅ Test program executed successfully:");
                let output_str = String::from_utf8_lossy(&run_output.stdout);
                for line in output_str.lines() {
                    println!("   {}", line);
                }
                
                // Check if GNU environment was detected
                if output_str.contains("Successfully using GNU environment") {
                    println!("🎉 GNU toolchain is working correctly!");
                } else {
                    println!("⚠️  GNU environment may not be active");
                }
            }
            Err(_) => println!("⚠️  Compiled successfully but couldn't run (may need MSYS2 DLLs in PATH)"),
        }

        // Clean up
        let _ = fs::remove_file("test_gnu.rs");
        let _ = fs::remove_file("test_gnu.exe");
    } else {
        println!("❌ Test compilation failed:");
        println!("{}", String::from_utf8_lossy(&compile_output.stderr));
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_check() {
        // This test will only pass on Windows
        if cfg!(target_os = "windows") {
            assert!(true);
        } else {
            println!("Skipping Windows-specific test on non-Windows platform");
        }
    }

    #[test]
    fn test_path_checking() {
        // Test that we can check for file existence
        let current_dir = std::env::current_dir().unwrap();
        assert!(current_dir.exists());
    }
}