use std::process::Command;

fn main() {
    let mut list_dir = Command::new("ls");

    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory.
    list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}
