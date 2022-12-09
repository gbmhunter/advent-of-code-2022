use std::fs;

pub fn run() {
    println!("day3");
    let contents = fs::read_to_string("src/day3.txt").unwrap();
    let lines = contents.lines();

    let mut shared_letters = Vec::new();
    for (_, line) in lines.clone().enumerate() {
        let compartment1 = &line[0..line.len() / 2];
        let compartment2 = &line[line.len() / 2..];
        for char in compartment1.chars() {
            if compartment2.contains(char) {
                // We are told there is only ever one shared letter, and so
                // we can bail once the first instance of it is found
                shared_letters.push(char);
                break;
            }
        }
    }

    // Convert letters to score
    let mut priority_sum = 0;
    for letter in shared_letters {
        priority_sum += letter_to_priority(&letter);
    }
    println!("part 1: priority_sum={}", priority_sum);

    // PART 2
    let lines_vec: Vec<&str> = lines.collect();
    priority_sum = 0;
    for i in 0..lines_vec.len() / 3 {
        let rucksack1 = lines_vec[i * 3 + 0];
        let rucksack2 = lines_vec[i * 3 + 1];
        let rucksack3 = lines_vec[i * 3 + 2];

        for char in rucksack1.chars() {
            if rucksack2.contains(char) && rucksack3.contains(char) {
                // We found it!
                priority_sum += letter_to_priority(&char);
                break;
            }
        }
    }
    println!("part 2: priority_sum={}", priority_sum);
}

pub fn letter_to_priority(letter: &char) -> u32 {
    if letter.is_ascii_lowercase() {
        *letter as u32 - ('a' as u32) + 1
    } else {
        *letter as u32 - 'A' as u32 + 1 + 26
    }
}
