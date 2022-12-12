use std::{fs, collections::HashMap};

pub fn run() {
    let contents = fs::read_to_string("src/day7.txt").unwrap();

    let dir_sizes: HashMap<&str, u32> = HashMap::new();
    let mut curr_path = String::from("/");
    for line in contents.lines() {
        println!("Current line={:?}", line);
        if line.starts_with("$ cd") {
            println!("Found cd command.");
            let (_, path_part) = line.split_at(5);
            println!("path_part={}", path_part);
            if path_part == "/" {
                curr_path = String::from("/");
            } else if path_part == ".." {
                println!("Found .., going to parent dir.");
                let mut bits: Vec<&str> = curr_path.split("/").collect();
                bits = bits[0..bits.len() - 2].to_vec();
                let parent_path = bits.join("/");
                curr_path = parent_path.to_string() + "/";
            } else {
                curr_path = curr_path + path_part + "/";
            }
            println!("curr_path is now: {:?}", curr_path);
        } else if line.starts_with("$ ls") {
            println!("Found ls command.")
        } else {
            // Must be output of ls command
            if line.starts_with("dir ") {
                println!("Found dir in ls output, ignoring.");
                continue;
            } else {
                // Must be a file
                let pieces: Vec<&str> = line.split(" ").collect();
                let file_size = pieces[0].parse::<u32>().unwrap();
                println!("file_size={}", file_size);
                increment_dir_sizes(&dir_sizes, &curr_path, file_size);
            }
        }
    }
}

fn increment_dir_sizes(dirs: &HashMap<&str, u32>, dir_path: &str, size: u32) {
    let pieces: Vec<&str> = dir_path.split("/").collect();
    println!("dir_path={:?}", dir_path);
    println!("pieces={:?}", pieces);
    let mut curr_path = "/";
    for i in 0..pieces.len() - 1 {
        let piece = pieces[i];
        println!("piece={}", piece);
    }
}