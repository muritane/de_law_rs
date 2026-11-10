use std::process::Command;

#[test]
fn command_renders_a_transient_result() {
    let output = Command::new(env!("CARGO_BIN_EXE_de-law"))
        .args(["majority", "--born", "2008-11-10", "--on", "2026-11-10"])
        .output()
        .expect("run the de-law CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rule=de.bgb.2.majority"));
    assert!(stdout.contains("status=adult"));
    assert!(stdout.contains("https://www.gesetze-im-internet.de/bgb/__2.html"));
}

#[test]
fn bad_input_fails_instead_of_guessing() {
    let output = Command::new(env!("CARGO_BIN_EXE_de-law"))
        .args(["majority", "--born", "2008-02-30", "--on", "2026-11-10"])
        .output()
        .expect("run the de-law CLI");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("date is outside"));
}
