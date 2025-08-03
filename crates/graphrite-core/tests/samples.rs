use std::process::Command;

fn run_cli(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("cargo").args(["run","-q","-p","graphrite-cli","--"]).args(args).output().expect("run cli");
    (output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout).into_owned(), String::from_utf8_lossy(&output.stderr).into_owned())
}

#[test]
fn validate_all_valid_samples() {
    let samples_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../samples/valid");
    for entry in std::fs::read_dir(&samples_dir).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().and_then(|s| s.to_str()) != Some("mmd") { continue; }
        let (code, _out, err) = run_cli(&["check", p.to_str().unwrap()]);
        assert_eq!(code, 0, "sample failed: {:?}\n{}", p, err);
    }
}
