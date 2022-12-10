use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day7.txt").unwrap();

    let mut curr_path = "/";
    for line in contents.lines() {
        println!("Current line={:?}", line);
        if line.starts_with("$ cd") {
            println!("Found cd command.");
        } else if line.starts_with("$ ls") {
            println!("Found ls command.")
        }
    }
}