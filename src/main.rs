use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", "Usage: {cgs <git_subdirectory>}");
    }

    let query = &args[1];

    let mut command = "git clone ".to_owned();
    command += query;
    // !NOTE: command = "git clone https://github.com/vercel/next.js/tree/deprecated-main/examples/active-class-name"

    let indexes_of_slash: Vec<_> = command[29..].match_indices("/").collect();
    let index_of_subdirectory = indexes_of_slash[1].0;

    let repository_url = &command[10..29 + index_of_subdirectory];
    println!("{:?}", repository_url);

    let mut list_dir = Command::new("ls");

    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory.
    list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}
