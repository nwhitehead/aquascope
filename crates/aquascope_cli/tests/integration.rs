
    
use assert_cmd::Command;

#[test]
fn invoke_help() {
    let mut cmd = Command::cargo_bin("aquascope_cli").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}
