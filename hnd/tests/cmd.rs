use assert_cmd::Command;

#[test]
fn run_with_no_args() {
    let mut cmd = Command::cargo_bin("hnd").unwrap();
    let assert = cmd.assert();
    assert.stdout("");
}
