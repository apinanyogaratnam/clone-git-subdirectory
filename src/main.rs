use std::process::Command;
use std::env;

fn execute_command(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", "Usage: {cgs <git_subdirectory>}");
    }

    let query = &args[1];

    let mut command = "git clone ".to_owned();
    command += query;

    let indexes_of_slash: Vec<_> = command[29..].match_indices("/").collect();
    let index_of_subdirectory = indexes_of_slash[1].0;

    let repository_url = &command[10..29 + index_of_subdirectory];

    command = command[..29+index_of_subdirectory].to_string();

    execute_command(&command);

    // // Change `ls` to execute in the root directory.
    // list_dir.current_dir("/");

    // // And then execute `ls` again but in the root directory.
    // list_dir.status().expect("process failed to execute");
}
