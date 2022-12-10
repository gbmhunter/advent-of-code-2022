use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day6.txt").unwrap();

    let mut num_chars = find_unique_seq(&contents, 4);
    println!("part 1: num_chars={:?}", num_chars);

    num_chars = find_unique_seq(&contents, 14);
    println!("part 2: num_chars={:?}", num_chars);
}

fn find_unique_seq(message: &str, num_unique_chars: usize) -> usize {
    for i in 0..message.len() - num_unique_chars + 1 {
        let slice = &message[i..i+num_unique_chars];
        let mut slice_vec = slice.as_bytes().to_vec();
        slice_vec.sort();
        slice_vec.dedup();
        if slice_vec.len() == num_unique_chars {
            return i + 4
        }
    }
    return 0;
}