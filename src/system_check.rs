use std::fs;
use anyhow::{Result, bail};

pub fn ensure_supported_environment() -> Result<()> {
    check_os()?;
    check_kernel()?;
    check_init_system()?;
    Ok(())
}

fn check_os() -> Result<()> {
    let os_release = fs::read_to_string("/etc/os-release")?;

    if !os_release.contains("ID=") {
        bail!("Cannot determine OS from /etc/os-release");
    }

    println!("OS detected. {os_release}");
    Ok(())
}

fn check_kernel() -> Result<()> {
    let version = fs::read_to_string("/proc/version")?;

    if !version.contains("Linux") {
        bail!("Not running Linux kernel.");
    }

    println!("Linux kernel detected.");
    Ok(())
}

fn check_init_system() -> Result<()> {
    let init = fs::read_to_string("/proc/1/comm")?;
    let init = init.trim();

    if init != "systemd" {
        bail!("Unsuppoported init system: {}", init);
    }

    println!("systemd detected.");
    Ok(())
}
