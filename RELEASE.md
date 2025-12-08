# Release Process

This document describes how to create releases for Pickup-Lang.

## Types of Releases

### Nightly Builds (Automatic)

**Nightly builds are created automatically on every push to the main branch.**

- A `nightly` pre-release is automatically updated with the latest binaries
- This happens automatically through GitHub Actions
- Binaries are available immediately after CI completes
- Access at: [Releases page](https://github.com/godofthunder8756/Pickup-Lang/releases)

### Versioned Releases (Manual)

For stable, versioned releases, follow the process below.

## Prerequisites

- Write access to the repository
- Git installed locally
- Changes you want to release are merged to the main branch

## Creating a Release

1. **Update the version number** in `Cargo.toml`:
   ```toml
   [package]
   version = "X.Y.Z"  # Update this line
   ```

2. **Commit the version change**:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to vX.Y.Z"
   git push origin main
   ```

3. **Create and push a version tag**:
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

4. **Wait for the release to build**: 
   - Go to the [Actions tab](https://github.com/godofthunder8756/Pickup-Lang/actions)
   - Watch the "Release" workflow
   - It will build binaries for all platforms (takes ~10-15 minutes)

5. **Verify the release**:
   - Go to the [Releases page](https://github.com/godofthunder8756/Pickup-Lang/releases)
   - The new release should be published with all binary assets

## Versioning

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (X.0.0): Incompatible API changes
- **MINOR** version (0.X.0): New functionality, backward compatible
- **PATCH** version (0.0.X): Backward compatible bug fixes

## Platform Support

Each release includes binaries for:

- **Linux**:
  - `pick-linux-x86_64` (glibc)
  - `pick-linux-x86_64-musl` (musl, more portable)
- **macOS**:
  - `pick-macos-x86_64` (Intel Macs)
  - `pick-macos-aarch64` (Apple Silicon M1/M2/M3)
- **Windows**:
  - `pick-windows-x86_64.exe`

## Troubleshooting

**Release workflow failed?**
- Check the [Actions tab](https://github.com/godofthunder8756/Pickup-Lang/actions) for error details
- Common issues:
  - Build failures: Fix the code and create a new patch version
  - Missing permissions: Ensure GitHub Actions has release permissions

**Need to delete a release?**
- Go to the Releases page
- Click on the release you want to delete
- Click "Delete this release"
- Delete the corresponding tag: `git push --delete origin vX.Y.Z`

## Testing Locally

Before creating a release, test the build process locally:

```bash
# Test the release build
cargo build --release

# Test cross-compilation (if you have the toolchains installed)
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```
