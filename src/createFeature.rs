use std::process::Command;

fn createFeature(name: String) {
    let subfolders = vec!["bin", "doc", "src", "test", "lib", "config"];
    let mut command: Command::new("mkdir")
        .arg(name)
        .output()
        .expect("failed to create directory");

    command = Command::new("cd")
        .arg(name)
        .output()
        .expect("failed to change directory");

    for folder in subfolders.iter() {
        command = Command::new("mkdir")
            .arg(folder)
            .output()
            .expect("failed to create directory");
    }
}
