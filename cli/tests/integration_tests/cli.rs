use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_set_integration() {
    let mut cmd = Command::cargo_bin("your_crate_name").unwrap();
    cmd.arg("SET").arg("mykey").arg("myvalue");
    cmd.assert()
        .success()
        .stdout(contains("Setting key 'mykey' to value 'myvalue'"));
}

#[test]
fn test_get_integration() {
    let mut cmd = Command::cargo_bin("your_crate_name").unwrap();
    cmd.arg("GET").arg("mykey");
    cmd.assert()
        .success()
        .stdout(contains("Getting value for key 'mykey'"));
}

#[test]
fn test_delete_integration() {
    let mut cmd = Command::cargo_bin("your_crate_name").unwrap();
    cmd.arg("DELETE").arg("mykey");
    cmd.assert()
        .success()
        .stdout(contains("Deleting key 'mykey'"));
}