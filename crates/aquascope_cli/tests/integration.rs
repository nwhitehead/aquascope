
    
use assert_cmd::Command;
use serde_json::Value;
use tempfile::NamedTempFile;
use anyhow::{Result, bail};
use std::fs;
use std::io::Write;

fn test(contents: &str) -> Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(contents.as_bytes())?;
    let mut cmd = Command::cargo_bin("aquascope_cli").unwrap();
    cmd.arg("--filename");
    cmd.arg(file.path());
    cmd.assert().success();

    Ok(())
}

#[test]
fn invoke_help() {
    let mut cmd = Command::cargo_bin("aquascope_cli").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn examples() -> Result<()> {
    test(r#"
fn main() {
  let mut x = 1;
  let y = x;
  x += 1;
}"#)?;

    // let data = String::from_utf8(output.stdout)?;
    // let v: Value = serde_json::from_str(&data)?;

    // assert that the contents of ./testdata/example.golden
    // are equal to `text`
    // goldie::assert!(text)
    Ok(())
}
