//!
//! The zkEVM LLVM builder library.
//!

pub mod llvm_path;
pub mod platforms;
pub mod utils;
pub mod lock;

pub use self::lock::Lock;
pub use self::llvm_path::LLVMPath;

use std::path::PathBuf;
use std::process::Command;


///
/// Clones the LLVM repository.
///
pub fn clone(lock: Lock) -> anyhow::Result<()> {
    utils::check_presence("git")?;

    let destination_path = PathBuf::from(LLVMPath::DIRECTORY_LLVM_SOURCE);
    if !destination_path.exists() {
        utils::command(
            Command::new("git").args([
                "clone",
                "--branch",
                lock.branch.as_str(),
                lock.url.as_str(),
                destination_path.to_string_lossy().as_ref(),
            ]),
            "LLVM repository cloning",
        )?;
    } else {
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["fetch", "--all", "--tags"]),
            "LLVM repository fetching",
        )?;
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["clean", "-d", "-x", "--force"]),
            "LLVM repository checking out",
        )?;
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["checkout", "--force", lock.branch.as_str()]),
            "LLVM repository checking out",
        )?;
    }

    Ok(())
}

///
/// Executes the LLVM building.
///
pub fn build() -> anyhow::Result<()> {
    std::fs::create_dir_all(LLVMPath::DIRECTORY_LLVM_TARGET)?;

    if cfg!(target_arch = "x86_64") {
        if cfg!(target_os = "linux") {
            if cfg!(target_env = "gnu") {
                platforms::x86_64_linux_gnu::build()?;
            } else if cfg!(target_env = "musl") {
                platforms::x86_64_linux_musl::build()?;
            }
        } else if cfg!(target_os = "macos") {
            platforms::x86_64_macos::build()?;
        } else if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
            platforms::x86_64_windows_gnu::build()?;
        }
    } else if cfg!(target_arch = "aarch64") {
        if cfg!(target_os = "macos") {
            platforms::aarch64_macos::build()?;
        }
    } else {
        anyhow::bail!("Unsupported on your machine");
    }

    Ok(())
}
