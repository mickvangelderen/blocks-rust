fn main() {
	  let cmd = std::process::Command::new("git").args(&["describe", "--always", "--abbrev=10", "--dirty"]).output().unwrap();
    assert!(cmd.status.success());
    let git_hash = std::str::from_utf8(&cmd.stdout[..]).unwrap();
	  println!("cargo:rustc-env={}={}", "GIT_HASH", git_hash);
}
