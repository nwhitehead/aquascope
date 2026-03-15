use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::{Result, bail};
use aquascope_workspace_utils::{miri_sysroot, run_and_get_output, rustc};
use clap::Parser;
use log::{Level, debug, error, info, log_enabled};
use tempfile::tempdir;

const TIMEOUT: u64 = 10;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program source file
    #[arg(short, long)]
    filename: String,

    /// Whether to expect program to fail
    #[arg(short, long, default_value_t = false)]
    should_fail: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let miri_sysroot = miri_sysroot().expect("Need MIRI sysroot");
    let rustc = rustc().expect("Need rustc");
    let target_libdir_output =
        run_and_get_output(Command::new(rustc.clone()).args(["--print", "target-libdir"]))
            .expect("Need libdir");
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

    fs::write(root.join("example/src/main.rs"), "fn main() { let x = 5; }\n")?;

    // let mut responses = HashMap::new();
    let mut cmd = Command::new("cargo");
    cmd.arg("aquascope")
        .env("SYSROOT", &miri_sysroot)
        .env("MIRI_SYSROOT", &miri_sysroot)
        .env("DYLD_LIBRARY_PATH", &target_libdir)
        .env("LD_LIBRARY_PATH", &target_libdir)
        .env("RUST_BACKTRACE", "1")
        .current_dir(root.join("example"));

    if args.should_fail {
        cmd.arg("--should-fail");
    }

    cmd.arg("interpreter");

    info!("cmd={:?}", cmd);

    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let output = child.wait_with_output()?;

    info!("output={:?}", output);
    Ok(())
}
