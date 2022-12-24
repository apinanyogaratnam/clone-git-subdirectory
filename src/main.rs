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

    let mut username_and_repo = &command[29 + 1..29 + index_of_subdirectory];
    let index_of_username = username_and_repo.find("/").unwrap();
    username_and_repo = &username_and_repo[index_of_username + 1..];

    let repository_subdirectory = &command[29 + index_of_subdirectory + 6..];

    let index_of_branch = repository_subdirectory.find("/").unwrap();
    let branch: &str = &repository_subdirectory[..index_of_branch];

    let mut checkout_branch_command = "git checkout ".to_owned();
    checkout_branch_command += branch;

    let subdirectory = &repository_subdirectory[index_of_branch + 1..];

    // let mut change_directory_command = "cd ".to_owned();
    // change_directory_command += &subdirectory;
    // println!("{}", change_directory_command);

    // let number_of_changes = subdirectory.matches("/").count() + 1;

    // let mut change_back_directory_command = "cd ".to_owned();
    // for _ in 0..number_of_changes {
    //     change_back_directory_command += "../";
    // }
    // println!("{}", change_back_directory_command);

    let mut move_command = "mv ".to_owned();
    move_command += &subdirectory;
    move_command += &" .";
    println!("{}", move_command);

    let mut remove_original_directory_command = "rm -rf ".to_owned();
    remove_original_directory_command += &username_and_repo;
    println!("{}", remove_original_directory_command);

    command = command[..29+index_of_subdirectory].to_string();

    println!("cloning...");

    execute_command(&command); // clone the repository
    execute_command(&checkout_branch_command); // checkout the branch
    execute_command(&move_command); // move the subdirectory to the current directory
    execute_command(&remove_original_directory_command); // remove the original directory
}
