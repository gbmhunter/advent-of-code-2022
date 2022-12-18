use std::fs;

pub fn run() {
    println!("day13");
    let contents = fs::read_to_string("src/day13.txt").unwrap();

    // PART 1
    let lines = contents.lines().collect::<Vec<_>>();

    let mut row_num = 0;
    let mut right_order_indices: Vec<usize> = Vec::new();
    while row_num < lines.len() {
        let mut left = String::from(lines[row_num]);
        let mut right = String::from(lines[row_num + 1]);
        let compare_result = compare(&mut left, &mut right);
        match compare_result {
            CompareResult::RightOrder => right_order_indices.push(row_num/3 + 1),
            _ => (),
        }
        row_num += 3; // Jump ahead to the next pair of lines in the input
    }
    let num_pairs_in_correct_order = right_order_indices.iter().sum::<usize>();
    println!("part 1: num. of pairs in the correct order = {:?}", num_pairs_in_correct_order);
    assert!(num_pairs_in_correct_order == 5330, "Incorrect answer.");

    // PART 2
    let mut lines_part_2 = lines.clone();
    // Remove all empty lines
    lines_part_2.retain(| line | *line != "");
    // Insert divider packets
    lines_part_2.push("[[2]]");
    lines_part_2.push("[[6]]");
    // Now sort
    lines_part_2.sort_by(|a, b| {
        let compare_result = compare(&mut a.to_string(), &mut b.to_string());
        match compare_result {
            CompareResult::RightOrder => return std::cmp::Ordering::Less,
            CompareResult::WrongOrder => return std::cmp::Ordering::Greater,
            _ => {
                panic!("Should not be here.");
            },
        }
    });

    let mut decoder_key = 1;
    for (usize, line) in lines_part_2.iter().enumerate() {
        if *line == "[[2]]" || *line == "[[6]]" {
            decoder_key *= usize + 1;
        }
    }
    println!("part 2: decoder key = {}", decoder_key);
    assert!(decoder_key == 27648, "Incorrect answer.");
}

#[derive(Debug)]
enum CompareResult {
    RightOrder,
    WrongOrder,
    Equal,
}

fn compare(left: &mut String, right: &mut String) -> CompareResult {
    let left_token = get_next_token(left);
    let right_token = get_next_token(right);

    if let (NextToken::ListStart, NextToken::ListStart) = (&left_token, &right_token) {
        // Neither list has finished, got to look at contents
        return compare(left, right);
    } else if let (NextToken::ListStop, NextToken::ListStop) = (&left_token, &right_token) {
        // Neither list has finished, got to look at contents
        return compare(left, right);
    }
    // ADD LIST HERE
    // LEFT -> List start, RIGHT -> Int 
    else if let (NextToken::ListStart, NextToken::Integer(right_int)) = (&left_token, &right_token) {
        // Convert int to list
        let string_to_insert = format!("{}]", right_int);
        right.insert_str(0, &string_to_insert);
        return compare(left, right);
    }
    // LEFT -> Int, RIGHT -> List start
    else if let (NextToken::Integer(left_int), NextToken::ListStart) = (&left_token, &right_token) {
        // Convert int to list
        left.insert_str(0, &format!("{}]", left_int));
        return compare(left, right);
    }

    // LEFT -> List end, RIGHT -> Int 
    else if let (NextToken::ListStop, NextToken::Integer(_)) = (&left_token, &right_token) {
        return CompareResult::RightOrder;
    }
    // LEFT -> Int, RIGHT -> List end
    else if let (NextToken::Integer(_), NextToken::ListStop) = (&left_token, &right_token) {
        return CompareResult::WrongOrder;
    }
    // LEFT -> List start, RIGHT -> List end 
    else if let (NextToken::ListStart, NextToken::ListStop) = (&left_token, &right_token) {
        return CompareResult::WrongOrder;
    }
    // LEFT -> List end, RIGHT -> List start
    else if let (NextToken::ListStop, NextToken::ListStart) = (&left_token, &right_token) {
        return CompareResult::RightOrder;
    }
    else if let (NextToken::Integer(left_int), NextToken::Integer(right_int)) = (&left_token, &right_token) {
        if left_int < right_int {
            return CompareResult::RightOrder;
        } else if left_int > right_int {
            return CompareResult::WrongOrder;
        } else {
            return compare(left, right);
        }
    }
    panic!("Should never get here.");
}

enum NextToken {
    ListStart,
    ListStop,
    Integer(u8),
}

fn get_next_token(line: &mut String) -> NextToken {
    if &line[0..1] == "[" {
        line.remove(0);
        return NextToken::ListStart;
    } else if &line[0..1] == "]" {
        line.remove(0);
        // If the next char is a comma, remove that too
        if line.len() >=1 && &line[0..1] == "," {
            // println!("Found comma");
            line.remove(0);
        }
        return NextToken::ListStop;
    } else if line[0..1].parse::<u8>().is_ok() {
        let mut int_as_string = String::from("");
        while line[0..1].parse::<u8>().is_ok() {
            int_as_string += &line[0..1];
            line.remove(0);
        }
        // println!("Found end of int! end={}", end_of_int);
        // Convert to int
        let int = int_as_string.parse::<u8>().unwrap();

        // If the next char is a comma, remove that too
        if &line[0..1] == "," {
            // println!("Found comma");
            line.remove(0);
        }
        return NextToken::Integer(int);
    }

    panic!("Should not have got here.");
}