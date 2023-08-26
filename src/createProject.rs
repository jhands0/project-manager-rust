use std::process::Command;

fn create_project(name: String) {
    let mut command = Command::new("mkdir")
        .args(name)
        .output()
        .expect("failed to create directory");

    command = Command::new("cd")
        .args(name)
        .output()
        .expect("failed to change directory");

    //let subfolders = vec!["bin", "doc", "src", "test", "lib", "config"];

    command = Command::new("git")
        .args("init")
        .output()
        .expect("failed to create git repository");
}
