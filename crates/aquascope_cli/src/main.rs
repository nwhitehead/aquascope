use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Result, bail};
use aquascope_workspace_utils::{miri_sysroot, run_and_get_output, rustc};
use clap::Parser;
use log::{error, info};
use serde_json::Value;
use tempfile::tempdir;

/// Invoke aquascope on standalone files and show output in JSON to stdout
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program source file
    #[arg(short = 'f', long)]
    filename: String,

    /// Whether to expect program to fail
    #[arg(short = 's', long, default_value_t = false)]
    should_fail: bool,

    /// Whether to include permission flows
    #[arg(short = 'p', long, default_value_t = false)]
    show_permissions: bool,
}

fn main() -> Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    // Indent multiline logs by 4 spaces
    builder.format_indent(Some(8)).init();

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

    // Now read input file and write to main.rs in example project.
    let contents = fs::read(args.filename)?;
    fs::write(root.join("example/src/main.rs"), &contents)?;

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

    let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let output = child.wait_with_output()?;

    info!("output={:?}", output);
    if !output.status.success() {
        error!("STDERR output:\n{}", String::from_utf8(output.stderr)?);
        error!("Aquascope invocation failed on {}", "main.rs");
        bail!("Aquascope failed");
    }
    // Success
    // Parse JSON, strip Ok part, stringify with indent
    let data = String::from_utf8(output.stdout)?;
    let v: Value = serde_json::from_str(&data)?;
    let mut v2 = v["Ok"].clone();
    v2["code"] = String::from_utf8(contents)?.into();
    let data_out = serde_json::to_string_pretty(&v2)?;
    println!("{}", data_out);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, bail};
    use assert_cmd::Command;
    use serde_json::Value;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn testit_args(name: &str, contents: &str, should_fail: bool) -> Result<Value> {
        let mut file = NamedTempFile::new()?;
        file.write_all(contents.as_bytes())?;
        let mut cmd = Command::cargo_bin("aquascope_cli").unwrap();
        if should_fail {
            cmd.arg("--should-fail");
        }
        cmd.arg("--filename");
        cmd.arg(file.path());
        let binding = cmd.assert();
        let output = binding.get_output();
        let outstring = String::from_utf8(output.stdout.clone())?;
        cmd.assert().success();
        let value: Value = serde_json::from_str(&outstring)?;
        goldie::new!().name(name).build().assert_json(value.clone());
        Ok(value)
    }

    fn testit(name: &str, contents: &str) -> Result<Value> {
        testit_args(name, contents, false)
    }

    fn testit_error(name: &str, contents: &str) -> Result<Value> {
        testit_args(name, contents, true)
    }

    #[test]
    fn invoke_help() {
        let mut cmd = Command::cargo_bin("aquascope_cli").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
    }

    #[test]
    fn example_basic() -> Result<()> {
        testit(
            "basic",
            r#"
fn main() {
    let mut x = 1;
    let y = x;
    x += 1;
}"#,
        )?;
        Ok(())
    }

    #[test]
    fn example_box() -> Result<()> {
        testit(
            "box",
            r#"
fn main() {
  let mut x = Box::new(0);
  *x += 1;
  let y = x;
}"#,
        )?;
        Ok(())
    }

    #[test]
    fn example_closure() -> Result<()> {
        testit(
            "closure",
            r#"
fn main() {
  let x = 0;
  let f = || x;
  let y = f();
}"#,
        )?;
        Ok(())
    }

        #[test]
    fn example_error() -> Result<()> {
        testit_error(
            "error",
            r#"
fn main() {
  let mut v = vec![1, 2, 3];
  let y = &v[0];
  v.push(0);
  let n = *y;
}"#,
        )?;
        Ok(())
    }

    #[test]
    fn example_nested_ref() -> Result<()> {
        testit(
            "nested_ref",
            r#"
fn main() {
  let x = [(0, 1), (2, 3)];
  let y = &x[1].1;
}"#,
        )?;
        Ok(())
    }

    #[test]
    fn example_stackref() -> Result<()> {
        testit(
            "stackref",
            r#"
fn main() {
  let mut x = 1;
  let y = &mut x;
  *y += 1;
}"#,
        )?;
        Ok(())
    }

    #[test]
    fn example_tuple() -> Result<()> {
        testit(
            "tuple",
            r#"
fn main() {
  let mut x = (0, String::from("Hello"));
  x.0 += 1;
}"#,
        )?;
        Ok(())
    }

}
