use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Result, bail};
use aquascope_workspace_utils::{miri_sysroot, run_and_get_output, rustc};
use log::{Level, debug, error, info, log_enabled};
use tempfile::tempdir;

fn main() -> Result<()> {
    env_logger::init();
    info!("Hello, world!");
    let miri_sysroot = miri_sysroot()?;
    let rustc = rustc()?;
    let target_libdir_output =
        run_and_get_output(Command::new(rustc.clone()).args(["--print", "target-libdir"]))?;
    let target_libdir = PathBuf::from(target_libdir_output);
    let tempdir = tempdir()?;
    let root = tempdir.path();

    info!("SYSROOT={}", miri_sysroot.display());
    info!("RUSTC={}", rustc.display());
    info!("LD_LIBRARY_PATH={}", target_libdir.display());
    info!("ROOT={}", root.display());

    // use `cargo new` to create new empty rust project
    let status = Command::new("cargo")
        .args(["new", "--bin", "example"])
        .current_dir(root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        bail!("Cargo failed");
    }

    Ok(())
}
