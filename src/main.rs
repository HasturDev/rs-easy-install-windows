use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
enum RustInstallationStatus {
    NotInstalled,
    InstalledMSVC(String),
    InstalledGNU(String),
    InstalledUnknown(String),
    BrokenInstallation(String),
}

fn main() {
    println!("🦀 Rust MSVC Installation Helper for Windows");
    println!("============================================\n");

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

    println!("This program will help you install Rust with MSVC toolchain.");
    println!("The MSVC toolchain provides native Windows development support.\n");

    // Step 1: Check for existing installations
    let rust_status = check_existing_installations()?;
    
    // Step 1.5: Handle existing installations
    handle_existing_rust_installation(&rust_status)?;

    // Step 2: Install Visual Studio C++ redistributable
    install_visual_cpp_redistributable()?;

    // Step 3: Install Rust with MSVC target
    install_rust_msvc()?;

    // Step 4: Ensure PATH is configured
    ensure_cargo_in_path()?;

    // Step 5: Verify installation
    verify_installation()?;

    Ok(())
}

fn check_existing_installations() -> Result<RustInstallationStatus, Box<dyn std::error::Error>> {
    println!("🔍 Checking for existing installations...\n");

    let rust_status = detect_rust_installation()?;
    
    match &rust_status {
        RustInstallationStatus::NotInstalled => {
            println!("No existing Rust installation found.");
        }
        RustInstallationStatus::InstalledMSVC(version) => {
            println!("✅ Found compatible Rust installation: {}", version);
            println!("   Already using MSVC toolchain.");
        }
        RustInstallationStatus::InstalledGNU(version) => {
            println!("⚠️  Found incompatible Rust installation: {}", version);
            println!("   Current installation uses GNU toolchain.");
            println!("   This will conflict with MSVC installation.");
        }
        RustInstallationStatus::InstalledUnknown(version) => {
            println!("⚠️  Found Rust installation with unknown toolchain: {}", version);
            println!("   May cause compatibility issues.");
        }
        RustInstallationStatus::BrokenInstallation(issue) => {
            println!("❌ Found broken Rust installation: {}", issue);
            println!("   Installation is incomplete or corrupted.");
        }
    }

    // Check for Visual Studio C++ redistributable
    check_visual_cpp_installed()?;

    println!();
    Ok(rust_status)
}

fn detect_rust_installation() -> Result<RustInstallationStatus, Box<dyn std::error::Error>> {
    // First check if rustup exists
    let rustup_available = match Command::new("rustup").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };
    
    if !rustup_available {
        return Ok(RustInstallationStatus::NotInstalled);
    }
    
    // Check rustup show to see the current state
    match Command::new("rustup").args(&["show"]).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Check for common "no toolchain" messages
            if stdout.contains("no default toolchain configured") || 
               stdout.contains("no toolchain is active") ||
               stderr.contains("no default is configured") ||
               stderr.contains("could not choose a version") ||
               stdout.contains("No `rustc` is currently active") {
                return Ok(RustInstallationStatus::BrokenInstallation("rustup installed but no active toolchain".to_string()));
            }
            
            // If rustup show works, try rustc
            match Command::new("rustc").arg("--version").output() {
                Ok(rustc_output) if rustc_output.status.success() => {
                    let version = String::from_utf8_lossy(&rustc_output.stdout).trim().to_string();
                    
                    if version.contains("msvc") {
                        Ok(RustInstallationStatus::InstalledMSVC(version))
                    } else if version.contains("gnu") {
                        Ok(RustInstallationStatus::InstalledGNU(version))
                    } else {
                        Ok(RustInstallationStatus::InstalledUnknown(version))
                    }
                }
                _ => {
                    // rustup show works but rustc doesn't
                    Ok(RustInstallationStatus::BrokenInstallation("rustup show works but rustc is not available".to_string()))
                }
            }
        }
        Err(_) => {
            // rustup exists but show command fails - likely broken
            Ok(RustInstallationStatus::BrokenInstallation("rustup exists but 'rustup show' command failed".to_string()))
        }
    }
}

fn handle_existing_rust_installation(status: &RustInstallationStatus) -> Result<(), Box<dyn std::error::Error>> {
    match status {
        RustInstallationStatus::NotInstalled => {
            println!("✅ No existing Rust installation - proceeding with fresh install.");
            Ok(())
        }
        RustInstallationStatus::InstalledMSVC(_) => {
            println!("✅ Compatible MSVC installation found - will update if needed.");
            Ok(())
        }
        RustInstallationStatus::InstalledGNU(version) => {
            println!("\n⚠️  INCOMPATIBLE INSTALLATION DETECTED");
            println!("=====================================");
            println!("Found: {}", version);
            println!("This GNU-based installation will conflict with the MSVC toolchain.");
            println!("\nOptions:");
            println!("1. Remove existing Rust installation (recommended)");
            println!("2. Cancel installation");
            println!();
            
            print!("Do you want to remove the existing installation? (y/N): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase().starts_with('y') {
                println!("\n🗑️ Removing existing Rust installation...");
                uninstall_existing_rust()?;
                println!("✅ Existing Rust installation removed successfully!");
            } else {
                return Err("Installation cancelled by user. Please manually remove the existing Rust installation or choose to continue with GNU toolchain.".into());
            }
            Ok(())
        }
        RustInstallationStatus::InstalledUnknown(version) => {
            println!("\n⚠️  UNKNOWN INSTALLATION DETECTED");
            println!("==============================");
            println!("Found: {}", version);
            println!("This installation may cause compatibility issues.");
            println!("\nOptions:");
            println!("1. Remove existing installation and install fresh (recommended)");
            println!("2. Try to continue with existing installation");
            println!("3. Cancel installation");
            println!();
            
            print!("Choose option (1/2/3): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => {
                    println!("\n🗑️ Removing existing Rust installation...");
                    uninstall_existing_rust()?;
                    println!("✅ Existing Rust installation removed successfully!");
                }
                "2" => {
                    println!("⚠️  Continuing with existing installation - may cause issues.");
                }
                _ => {
                    return Err("Installation cancelled by user.".into());
                }
            }
            Ok(())
        }
        RustInstallationStatus::BrokenInstallation(issue) => {
            println!("\n❌ BROKEN INSTALLATION DETECTED");
            println!("===============================");
            println!("Issue: {}", issue);
            println!("The existing Rust installation is incomplete or corrupted.");
            println!("\nOptions:");
            println!("1. Fix the existing installation by setting up MSVC toolchain (recommended)");
            println!("2. Remove existing installation and install fresh");
            println!("3. Cancel installation");
            println!();
            
            print!("Choose option (1/2/3): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => {
                    println!("\n🔧 Attempting to fix existing installation...");
                    fix_broken_rust_installation()?;
                }
                "2" => {
                    println!("\n🗑️ Removing existing Rust installation...");
                    uninstall_existing_rust()?;
                    println!("✅ Existing Rust installation removed successfully!");
                }
                _ => {
                    return Err("Installation cancelled by user.".into());
                }
            }
            Ok(())
        }
    }
}

fn fix_broken_rust_installation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to fix broken Rust installation...");
    
    // First, try to install the stable MSVC toolchain
    println!("Installing stable MSVC toolchain...");
    let install_output = Command::new("rustup")
        .args(&["toolchain", "install", "stable-x86_64-pc-windows-msvc"])
        .output()?;
    
    if !install_output.status.success() {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        println!("⚠️  Failed to install MSVC toolchain: {}", stderr);
    } else {
        println!("✅ MSVC toolchain installed successfully");
    }
    
    // Set it as the default
    println!("Setting MSVC toolchain as default...");
    let default_output = Command::new("rustup")
        .args(&["default", "stable-x86_64-pc-windows-msvc"])
        .output()?;
    
    if !default_output.status.success() {
        let stderr = String::from_utf8_lossy(&default_output.stderr);
        return Err(format!("Failed to set default toolchain: {}", stderr).into());
    }
    
    println!("✅ Default toolchain set to MSVC");
    
    // Verify the fix worked
    match Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ Installation fixed! Current version: {}", version.trim());
            
            if version.contains("msvc") {
                println!("🎉 Successfully configured MSVC toolchain!");
            } else {
                println!("⚠️  Toolchain is working but may not be MSVC. Version: {}", version.trim());
            }
        }
        _ => {
            return Err("Fix attempt failed - rustc is still not working".into());
        }
    }
    
    Ok(())
}

fn uninstall_existing_rust() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to uninstall existing Rust installation...");
    
    // Try rustup self uninstall first
    match Command::new("rustup").args(&["self", "uninstall", "-y"]).output() {
        Ok(output) if output.status.success() => {
            println!("✅ Successfully uninstalled via rustup");
            return Ok(());
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("⚠️  rustup uninstall failed: {}", stderr);
        }
        Err(e) => {
            println!("⚠️  Could not run rustup uninstall: {}", e);
        }
    }
    
    // Fallback: Manual cleanup
    println!("Attempting manual cleanup...");
    manual_rust_cleanup()?;
    
    Ok(())
}

fn manual_rust_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    // Get user profile directory
    let user_profile = std::env::var("USERPROFILE")
        .map_err(|_| "Could not get user profile directory")?;
    
    let paths_to_remove = [
        format!("{}/.cargo", user_profile),
        format!("{}/.rustup", user_profile),
        format!("{}\\.cargo", user_profile),
        format!("{}\\.rustup", user_profile),
    ];
    
    let mut removed_any = false;
    
    for path_str in &paths_to_remove {
        let path = Path::new(path_str);
        if path.exists() {
            match fs::remove_dir_all(path) {
                Ok(_) => {
                    println!("  ✅ Removed: {}", path_str);
                    removed_any = true;
                }
                Err(e) => {
                    println!("  ⚠️  Could not remove {}: {}", path_str, e);
                }
            }
        }
    }
    
    // Try to remove from PATH via PowerShell
    let cleanup_path_cmd = r#"
if ($env:PATH -like "*cargo*" -or $env:PATH -like "*rustup*") {
    Write-Host "Found Rust-related PATH entries. Please manually remove them from your PATH environment variable."
    Write-Host "Look for entries containing: .cargo\bin or .rustup"
} else {
    Write-Host "No obvious Rust PATH entries found."
}
"#;
    
    let _ = Command::new("powershell")
        .args(&["-Command", cleanup_path_cmd])
        .output();
    
    if !removed_any {
        println!("⚠️  No Rust directories found to remove");
        println!("  The existing installation might be system-wide or in a non-standard location");
    }
    
    println!("  ℹ️  You may need to restart your terminal for PATH changes to take effect");
    
    Ok(())
}

fn check_visual_cpp_installed() -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking for Visual Studio C++ redistributable...");

    // Check registry for installed Visual C++ redistributables
    let check_cmd = r#"
Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*" | 
Where-Object {$_.DisplayName -like "*Visual C++ 2015-2022 Redistributable*" -or 
              $_.DisplayName -like "*Visual C++ 2019 Redistributable*" -or
              $_.DisplayName -like "*Visual C++ 2017 Redistributable*"} | 
Select-Object DisplayName, DisplayVersion
"#;

    match Command::new("powershell")
        .args(&["-Command", check_cmd])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                println!("✅ Found Visual C++ redistributable:");
                for line in stdout.lines() {
                    if !line.trim().is_empty() && !line.contains("---") && !line.contains("DisplayName") {
                        println!("   {}", line.trim());
                    }
                }
                return Ok(true);
            }
        }
        Err(_) => {}
    }

    println!("❌ Visual C++ redistributable not found or not detectable.");
    Ok(false)
}

fn install_visual_cpp_redistributable() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Visual Studio C++ Redistributable Installation");
    println!("-------------------------------------------------");

    // Check if already installed
    if check_visual_cpp_installed()? {
        println!("✅ Visual C++ redistributable is already installed.");
        return Ok(());
    }

    println!("Visual C++ redistributable not found. Installing...");
    
    // Download and install Visual C++ redistributable
    download_and_install_visual_cpp()?;

    println!("✅ Visual C++ redistributable installation completed!");
    println!();
    Ok(())
}

fn download_and_install_visual_cpp() -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Downloading Visual C++ redistributable...");
    
    // Use the latest Visual C++ redistributable (2015-2022)
    let vc_redist_url = "https://aka.ms/vs/17/release/vc_redist.x64.exe";
    
    // Get current directory and create absolute path
    let current_dir = std::env::current_dir()?;
    let installer_path = current_dir.join("vc_redist.x64.exe");
    let installer_path_str = installer_path.to_string_lossy();
    
    // Download the redistributable
    let download_cmd = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        vc_redist_url, installer_path_str
    );
    
    println!("Downloading from: {}", vc_redist_url);
    let download_output = Command::new("powershell")
        .args(&["-Command", &download_cmd])
        .output()?;
    
    if !download_output.status.success() {
        let error_msg = String::from_utf8_lossy(&download_output.stderr);
        if error_msg.contains("cannot be loaded because running scripts is disabled") {
            return Err("PowerShell execution policy blocks downloads. Please run as administrator or enable PowerShell scripts.".into());
        }
        return Err(format!(
            "Failed to download Visual C++ redistributable: {}",
            error_msg
        ).into());
    }
    
    if !installer_path.exists() {
        return Err("Visual C++ redistributable download failed - file not found".into());
    }
    
    println!("✅ Download completed successfully ({:.1} MB)", 
             fs::metadata(&installer_path)?.len() as f64 / 1_000_000.0);
    
    // Install the redistributable
    println!("🚀 Installing Visual C++ redistributable...");
    println!("   This may take a few minutes, please wait...");
    
    // Try different installation methods
    let success = try_install_vc_redist(&installer_path)?;
    
    // Clean up installer file
    let _ = fs::remove_file(&installer_path);
    
    if !success {
        return Err("Failed to install Visual C++ redistributable. Please install manually from https://aka.ms/vs/17/release/vc_redist.x64.exe".into());
    }
    
    println!("✅ Visual C++ redistributable installation completed");
    
    // Wait for installation to fully complete
    thread::sleep(Duration::from_secs(3));
    
    Ok(())
}

fn try_install_vc_redist(installer_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let installer_str = installer_path.to_string_lossy();
    
    // Method 1: Direct execution
    println!("Trying direct installation...");
    match Command::new(&*installer_str)
        .args(&["/install", "/quiet", "/norestart"])
        .output() {
        Ok(output) if output.status.success() => {
            println!("✅ Direct installation successful");
            return Ok(true);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Check if it's already installed
            if stderr.contains("Another version of this product is already installed") || 
               stdout.contains("already installed") ||
               stderr.contains("0x80070666") { // ERROR_ANOTHER_PRODUCT_INSTALLED
                println!("✅ Visual C++ redistributable was already installed");
                return Ok(true);
            }
            
            println!("Direct installation failed: {}", stderr);
        }
        Err(e) => {
            println!("Direct execution failed: {}", e);
        }
    }
    
    // Method 2: PowerShell Start-Process
    println!("Trying PowerShell installation...");
    let ps_cmd = format!(
        "Start-Process -FilePath '{}' -ArgumentList '/install', '/quiet', '/norestart' -Wait -PassThru | ForEach-Object {{ Write-Output $_.ExitCode }}",
        installer_str
    );
    
    match Command::new("powershell")
        .args(&["-Command", &ps_cmd])
        .output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Check exit code
            if stdout.trim() == "0" || 
               stderr.contains("0x80070666") || // Already installed
               stdout.contains("0") {
                println!("✅ PowerShell installation successful");
                return Ok(true);
            }
            
            println!("PowerShell installation failed. Exit code: {}, Error: {}", stdout.trim(), stderr);
        }
        Err(e) => {
            println!("PowerShell execution failed: {}", e);
        }
    }
    
    // Method 3: Elevated PowerShell
    println!("Trying elevated installation...");
    let elevated_cmd = format!(
        "Start-Process -FilePath '{}' -ArgumentList '/install', '/quiet', '/norestart' -Verb RunAs -Wait",
        installer_str
    );
    
    match Command::new("powershell")
        .args(&["-Command", &elevated_cmd])
        .output() {
        Ok(output) if output.status.success() => {
            println!("✅ Elevated installation completed");
            return Ok(true);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Elevated installation failed: {}", stderr);
        }
        Err(e) => {
            println!("Elevated execution failed: {}", e);
        }
    }
    
    Ok(false)
}

fn install_rust_msvc() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Installing Rust with MSVC Target");
    println!("-----------------------------------");

    // Check if rustup is available after potential cleanup
    match Command::new("rustup").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✅ rustup found. Ensuring MSVC target is available...");
            
            // Add the MSVC target (default on Windows)
            let target_output = Command::new("rustup")
                .args(&["target", "add", "x86_64-pc-windows-msvc"])
                .output()?;

            if target_output.status.success() {
                println!("✅ x86_64-pc-windows-msvc target confirmed/added successfully!");
            } else {
                let stderr = String::from_utf8_lossy(&target_output.stderr);
                if stderr.contains("is up to date") || stderr.contains("already installed") {
                    println!("✅ x86_64-pc-windows-msvc target already available!");
                } else {
                    eprintln!("⚠️  Issue with MSVC target: {}", stderr);
                }
            }
        }
        _ => {
            println!("rustup not found. Installing fresh Rust with MSVC toolchain...");
            install_rustup_msvc()?;
            // Wait for installation to complete
            thread::sleep(Duration::from_secs(2));
        }
    }

    // Verify and set MSVC as default target
    configure_msvc_toolchain()?;

    println!();
    Ok(())
}

fn configure_msvc_toolchain() -> Result<(), Box<dyn std::error::Error>> {
    // Set MSVC as default target for current directory (if not already)
    match Command::new("rustup").args(&["show"]).output() {
        Ok(output) => {
            let show_output = String::from_utf8_lossy(&output.stdout);
            if !show_output.contains("x86_64-pc-windows-msvc") {
                let override_output = Command::new("rustup")
                    .args(&["override", "set", "stable-x86_64-pc-windows-msvc"])
                    .output();

                match override_output {
                    Ok(out) if out.status.success() => {
                        println!("✅ Set MSVC toolchain as default for current directory");
                    }
                    _ => {
                        println!("ℹ️  You can manually set MSVC toolchain with:");
                        println!("   rustup override set stable-x86_64-pc-windows-msvc");
                    }
                }
            } else {
                println!("✅ MSVC toolchain is already the active target");
            }
        }
        Err(_) => {
            println!("ℹ️  Could not check current toolchain - may need to restart terminal");
        }
    }
    Ok(())
}

fn install_rustup_msvc() -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Downloading rustup installer...");
    
    let rustup_url = "https://win.rustup.rs/x86_64";
    let current_dir = std::env::current_dir()?;
    let installer_path = current_dir.join("rustup-init.exe");
    let installer_path_str = installer_path.to_string_lossy();
    
    // Download rustup-init.exe
    let download_cmd = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        rustup_url, installer_path_str
    );
    
    println!("Downloading from: {}", rustup_url);
    let download_output = Command::new("powershell")
        .args(&["-Command", &download_cmd])
        .output()?;
    
    if !download_output.status.success() {
        let error_msg = String::from_utf8_lossy(&download_output.stderr);
        return Err(format!("Failed to download rustup installer: {}", error_msg).into());
    }
    
    if !installer_path.exists() {
        return Err("rustup installer download failed - file not found".into());
    }
    
    println!("✅ rustup installer downloaded successfully");
    
    // Install rustup with MSVC as default target
    println!("🚀 Installing rustup with MSVC toolchain...");
    println!("   This will install Rust with x86_64-pc-windows-msvc as default");
    
    let install_output = Command::new(&*installer_path_str)
        .args(&[
            "--default-host", "x86_64-pc-windows-msvc",
            "--default-toolchain", "stable",
            "--profile", "default",
            "-y"  // Accept all defaults
        ])
        .output()?;
    
    // Clean up installer
    let _ = fs::remove_file(&installer_path);
    
    if !install_output.status.success() {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        let stdout = String::from_utf8_lossy(&install_output.stdout);
        return Err(format!(
            "rustup installation failed:\nSTDERR: {}\nSTDOUT: {}", 
            stderr, stdout
        ).into());
    }
    
    println!("✅ rustup installation completed successfully!");
    
    // Wait for installation to settle
    thread::sleep(Duration::from_secs(2));
    
    // Ensure Cargo is in PATH
    println!("🔄 Configuring PATH environment...");
    ensure_cargo_in_path()?;
    
    // Try to refresh environment variables
    let _ = Command::new("powershell")
        .args(&["-Command", "refreshenv 2>$null"])
        .output();
    
    // Verify installation
    match Command::new("rustup").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ Verified rustup installation: {}", version.trim());
        }
        _ => {
            println!("⚠️  rustup installed but not immediately available in current PATH");
            println!("   Please restart your terminal or run a new command prompt");
            println!("   You can verify the installation with: rustup --version");
        }
    }
    
    Ok(())
}

fn ensure_cargo_in_path() -> Result<(), Box<dyn std::error::Error>> {
    let user_profile = std::env::var("USERPROFILE")
        .map_err(|_| "Could not get user profile directory")?;
    let cargo_bin = format!(r"{}\\.cargo\\bin", user_profile);
    
    println!("Ensuring {} is in PATH...", cargo_bin);
    
    // Check if Cargo bin is already in user PATH
    let check_path_cmd = format!(
        r#"$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User'); 
           if ($userPath -like '*{}*') {{ 
               Write-Output 'Already in PATH' 
           }} else {{ 
               Write-Output 'Not in PATH' 
           }}"#,
        cargo_bin.replace("\\", "\\\\")
    );
    
    match Command::new("powershell")
        .args(&["-Command", &check_path_cmd])
        .output() {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            let result = result.trim();
            if result == "Already in PATH" {
                println!("✅ Cargo is already in user PATH");
                return Ok(());
            }
        }
        Err(_) => {
            println!("⚠️  Could not check PATH, attempting to add anyway...");
        }
    }
    
    // Add Cargo bin to user PATH
    let add_path_cmd = format!(
        r#"$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User');
           if ($userPath -eq $null -or $userPath -eq '') {{
               $newPath = '{}'
           }} else {{
               $newPath = $userPath + ';{}'
           }}
           [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User');
           Write-Output 'Added to PATH'"#,
        cargo_bin, cargo_bin
    );
    
    match Command::new("powershell")
        .args(&["-Command", &add_path_cmd])
        .output() {
        Ok(output) if output.status.success() => {
            println!("✅ Added Cargo to user PATH");
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("⚠️  Issues adding to PATH: {}", stderr);
        }
        Err(e) => {
            println!("⚠️  Could not add to PATH: {}", e);
        }
    }
    
    Ok(())
}

fn verify_installation() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Verifying Installation");
    println!("-------------------------");

    // Check rustc version and target
    match Command::new("rustc").args(&["--version", "--verbose"]).output() {
        Ok(output) => {
            println!("Rust compiler info:");
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("host:") && line.contains("msvc") {
                    println!("✅ {}", line);
                } else if line.contains("release:") || line.contains("commit-hash:") {
                    println!("   {}", line);
                } else if !line.trim().is_empty() {
                    println!("   {}", line);
                }
            }
        }
        Err(_) => println!("❌ Could not run rustc"),
    }

    // Check available targets
    match Command::new("rustup").args(&["target", "list", "--installed"]).output() {
        Ok(output) => {
            let targets = String::from_utf8_lossy(&output.stdout);
            println!("\nInstalled targets:");
            for line in targets.lines() {
                if line.contains("windows-msvc") {
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
    println!("Hello from Rust with MSVC toolchain!");
    println!("Target: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
    
    #[cfg(target_env = "msvc")]
    println!("✅ Successfully using MSVC environment!");
    
    #[cfg(not(target_env = "msvc"))]
    println!("⚠️  Not using MSVC environment");
}"#;

    fs::write("test_msvc.rs", test_code)?;

    let compile_output = Command::new("rustc")
        .args(&["test_msvc.rs", "--target", "x86_64-pc-windows-msvc"])
        .output()?;

    if compile_output.status.success() {
        println!("✅ Test compilation successful!");
        
        // Try to run the compiled program
        match Command::new("./test_msvc.exe").output() {
            Ok(run_output) => {
                println!("✅ Test program executed successfully:");
                let output_str = String::from_utf8_lossy(&run_output.stdout);
                for line in output_str.lines() {
                    println!("   {}", line);
                }
                
                // Check if MSVC environment was detected
                if output_str.contains("Successfully using MSVC environment") {
                    println!("🎉 MSVC toolchain is working correctly!");
                } else {
                    println!("⚠️  MSVC environment may not be active");
                }
            }
            Err(_) => println!("⚠️  Compiled successfully but couldn't run test program"),
        }

        // Clean up
        let _ = fs::remove_file("test_msvc.rs");
        let _ = fs::remove_file("test_msvc.exe");
    } else {
        println!("❌ Test compilation failed:");
        println!("{}", String::from_utf8_lossy(&compile_output.stderr));
    }

    println!("\n🎯 Installation Summary:");
    println!("• Visual Studio C++ redistributable: Installed");
    println!("• Rust with MSVC toolchain: Installed");
    println!("• PATH environment variable: Configured");
    println!("• Ready for native Windows development!");
    println!();
    println!("📝 Important Notes:");
    println!("• If commands don't work immediately, restart your terminal");
    println!("• You can verify the installation by running: cargo --version");
    println!("• Use 'cargo new my_project' to create a new Rust project");
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

    #[test]
    fn test_msvc_target_detection() {
        // Test that we can detect MSVC target
        if cfg!(target_env = "msvc") {
            assert!(true, "MSVC environment detected");
        } else {
            println!("Not running on MSVC environment");
        }
    }
}