use std::process::Command;
use std::io::Cursor;
use plist::Value;

fn main() {
    let args = [
        "/Applications/Firefox.app",
        "-d",
        "--entitlements",
        ":-",
    ];

    let output = Command::new("codesign")
        .args(&args)
        .output()
        .expect("Failed to execute command");

    // `codesign` may return 1 if file is not signed at all
    assert!(output.status.success(), "Application could not be parsed or is not signed");

    let entitlements_plist = Value::from_reader(Cursor::new(output.stdout))
        .expect("Unable to parse entitlements plist");

    let app_sandbox = entitlements_plist
        .as_dictionary()
        .and_then(|dict| dict.get("com.apple.security.app-sandbox"))
        .and_then(|value| value.as_boolean());
    println!("App sandbox: {:?}", app_sandbox);
}
