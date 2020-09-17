use super::errors::CargoResult;

pub mod download;

pub fn nix_version() -> CargoResult<String> {
    use std::process::{Command, Stdio};

    let version = Command::new("nix")
      .arg("--version")
      .stderr(Stdio::null())
      .stdout(Stdio::piped())
      .output()?;

    let version = String::from_utf8_lossy(&version.stdout);
    let version = version.to_string();
    log::trace!("found nix version: {}", version);
    Ok(version)
}

pub fn is_nix_installed() -> CargoResult<bool> {
    Ok(nix_version().is_ok())
}

#[cfg(os = "unix")]
pub fn is_nix_build() -> CargoResult<bool> {
    let home = std::env::var("HOME").context("Failed to read HOME variable")?;
    Ok(home == "/homeless-shelter")
}