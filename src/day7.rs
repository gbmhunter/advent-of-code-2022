use std::{fs, collections::HashMap};

pub fn run() {
    let contents = fs::read_to_string("src/day7.txt").unwrap();

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut curr_path = String::from("/");
    for line in contents.lines() {
        if line.starts_with("$ cd") {
            let (_, path_part) = line.split_at(5);
            if path_part == "/" {
                curr_path = String::from("/");
            } else if path_part == ".." {
                let mut bits: Vec<&str> = curr_path.split("/").collect();
                bits = bits[0..bits.len() - 2].to_vec();
                let parent_path = bits.join("/");
                curr_path = parent_path.to_string() + "/";
            } else {
                curr_path = curr_path + path_part + "/";
            }
        } else if line.starts_with("$ ls") {
        } else {
            // Must be output of ls command
            if line.starts_with("dir ") {
                continue;
            } else {
                // Must be a file
                let pieces: Vec<&str> = line.split(" ").collect();
                let file_size = pieces[0].parse::<u32>().unwrap();
                increment_dir_sizes(&mut dir_sizes, curr_path.clone(), file_size);
            }
        }
    }

    let mut total_size = 0;
    for (_, size) in &dir_sizes {
        if *size <= 100000 {
            total_size += size;
        }
    }
    println!("part 1: total_size={}", total_size);

    let total_disk_space = 70000000;
    let required_free_space = 30000000;
    let total_used_space = *dir_sizes.get("/").unwrap();
    let curr_free_space = total_disk_space - total_used_space;

    let min_space_we_need_to_free = required_free_space - curr_free_space;

    let mut best_dir_size = total_used_space;
    for (_, size) in &dir_sizes {
        if *size > min_space_we_need_to_free && *size < best_dir_size {
            best_dir_size = *size;
        }
    }

    println!("part 2: dir_size={}", best_dir_size);
}

fn increment_dir_sizes(dir_sizes: &mut HashMap<String, u32>, dir_path: String, size: u32) {
    let pieces: Vec<&str> = dir_path.split("/").collect();
    let mut curr_path = "/".to_string();
    for i in 0..pieces.len() - 1 {
        let piece = pieces[i];
        curr_path += piece;
        if !curr_path.ends_with("/") {
            curr_path += "/";
        }
        dir_sizes.entry(curr_path.clone()).and_modify(| old | *old += size).or_insert(size);
        
    }
}