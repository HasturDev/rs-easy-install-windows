# GitHub Action Workflow

This repository includes an automated GitHub Action that builds and releases the Rust MSVC Installer.

## Workflow Triggers

The workflow runs on:

### 1. **Automatic Triggers**
- ✅ **Push to `vs-c++-method` branch**: Builds the installer
- ✅ **Git tags starting with `v*`**: Builds and creates a release
- ✅ **Pull requests to `vs-c++-method`**: Builds for testing

### 2. **Manual Triggers**
- ✅ **Manual workflow dispatch**: Can be triggered from GitHub Actions tab
  - Option to create a release manually
  - Custom release tag specification

## What the Workflow Does

### Build Job
1. **Setup**: Installs Rust toolchain with MSVC target
2. **Quality**: Runs formatting, clippy, and tests
3. **Build**: Compiles release binary for Windows
4. **Verify**: Tests that the executable runs
5. **Artifacts**: Uploads build artifacts for download

### Release Job (when triggered)
1. **Download**: Gets build artifacts
2. **Release Notes**: Generates comprehensive release documentation
3. **GitHub Release**: Creates release with binaries and documentation
4. **Tagging**: Auto-tags with date/time if no tag provided

## File Structure

```
.github/
└── workflows/
    └── build-and-release.yml  # Main workflow file
```

## Using the Workflow

### To Create a Release

#### Option 1: Git Tag (Recommended)
```bash
git tag v1.0.0
git push origin v1.0.0
```

#### Option 2: Manual Dispatch
1. Go to the "Actions" tab in GitHub
2. Select "Build and Release Rust MSVC Installer"
3. Click "Run workflow"
4. Choose:
   - ✅ "Create a new release": true
   - 📝 "Release tag": v1.0.0 (or leave blank for auto-generated)

### To Just Build (No Release)
Push to the `vs-c++-method` branch - it will build and upload artifacts without creating a release.

## Workflow Features

- 🚀 **Cross-platform ready**: Runs on Windows for native compilation
- 📦 **Artifact caching**: Speeds up subsequent builds
- 🔍 **Quality checks**: Formatting, linting, and testing
- 📝 **Auto-documentation**: Generates release notes with build info
- 🏷️ **Smart tagging**: Auto-generates tags if none provided
- 💾 **Artifact storage**: 30-day retention for build outputs
- 📊 **Build summaries**: Detailed status reporting

## Artifacts

Each workflow run produces:

1. **rust-msvc-installer-windows**
   - `rs-easy-installer-windows.exe` - The main installer
   - `README.md` - Project documentation
   - `LICENSE` - License file

2. **build-info**
   - `build-info.md` - Build metadata and feature list

## Configuration

The workflow uses these environment variables:
- `CARGO_TERM_COLOR=always` - Colored cargo output
- `RUST_BACKTRACE=1` - Full backtraces for debugging

## Permissions

The workflow requires:
- **Contents: read** - To checkout code
- **Actions: read** - To download artifacts  
- **Releases: write** - To create GitHub releases (automatic with GITHUB_TOKEN)

## Status Badges

Add these to your README to show build status:

```markdown
[![Build Status](https://github.com/hastur-dev/rs-easy-installer-windows/actions/workflows/build-and-release.yml/badge.svg?branch=vs-c%2B%2B-method)](https://github.com/hastur-dev/rs-easy-installer-windows/actions/workflows/build-and-release.yml)

[![Latest Release](https://img.shields.io/github/v/release/hastur-dev/rs-easy-installer-windows)](https://github.com/hastur-dev/rs-easy-installer-windows/releases/latest)
```