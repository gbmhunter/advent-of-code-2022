use std::fs;

pub fn run() {
    println!("day13");
    let contents = fs::read_to_string("src/day13.txt").unwrap();
    println!("{}", contents);
    let lines = contents.lines().collect::<Vec<_>>();

    let mut row_num = 0;
    let mut right_order_indices: Vec<usize> = Vec::new();
    while row_num < lines.len() {
        let mut left = String::from(lines[row_num]);
        let mut right = String::from(lines[row_num + 1]);
        let compare_result = compare(&mut left, &mut right);
        println!("FINAL compare result = {:?}", compare_result);
        match compare_result {
            CompareResult::RightOrder => right_order_indices.push(row_num/3 + 1),
            _ => (),
        }
        row_num += 3; // Jump ahead to the next pair of lines in the input
    }
    println!("{:?}", right_order_indices);
    println!("{:?}", right_order_indices.iter().sum::<usize>());

}

#[derive(Debug)]
enum CompareResult {
    RightOrder,
    WrongOrder,
    Equal,
}

fn compare(left: &mut String, right: &mut String) -> CompareResult {
    println!("compare() called. left={}, right={}", left, right);
    let mut curr_index = 1;

    let left_token = get_next_token(left);
    let right_token = get_next_token(right);

    if let (NextToken::ListStart, NextToken::ListStart) = (&left_token, &right_token) {
        // Neither list has finished, got to look at contents
        println!("Both left and right are start of lists");
        return compare(left, right);
    } else if let (NextToken::ListStop, NextToken::ListStop) = (&left_token, &right_token) {
        // Neither list has finished, got to look at contents
        println!("Both left and right are ends of lists");
        return compare(left, right);
    }
    // ADD LIST HERE
    // LEFT -> List start, RIGHT -> Int 
    else if let (NextToken::ListStart, NextToken::Integer(right_int)) = (&left_token, &right_token) {
        println!("Left is list, right is int.");
        // Convert int to list
        println!("List before conversion: {}", right);
        let string_to_insert = format!("{}]", right_int);
        right.insert_str(0, &string_to_insert);
        println!("List after conversion: {}", right);
        return compare(left, right);
    }
    // LEFT -> Int, RIGHT -> List start
    else if let (NextToken::Integer(left_int), NextToken::ListStart) = (&left_token, &right_token) {
        println!("Left is int, right is list.");
        // Convert int to list
        left.insert_str(0, &format!("{}]", left_int));
        return compare(left, right);
    }

    // LEFT -> List end, RIGHT -> Int 
    else if let (NextToken::ListStop, NextToken::Integer(right_int)) = (&left_token, &right_token) {
        println!("Left is list end, right is int.");
        return CompareResult::RightOrder;
    }
    // LEFT -> Int, RIGHT -> List end
    else if let (NextToken::Integer(left_int), NextToken::ListStop) = (&left_token, &right_token) {
        println!("Left is int, right is list end.");
        return CompareResult::WrongOrder;
    }
    // LEFT -> List start, RIGHT -> List end 
    else if let (NextToken::ListStart, NextToken::ListStop) = (&left_token, &right_token) {
        println!("Left is list start, right is list end.");
        return CompareResult::WrongOrder;
    }
    // LEFT -> List end, RIGHT -> List start
    else if let (NextToken::ListStop, NextToken::ListStart) = (&left_token, &right_token) {
        println!("Left is list end, right is list start.");
        return CompareResult::RightOrder;
    }
    else if let (NextToken::Integer(left_int), NextToken::Integer(right_int)) = (&left_token, &right_token) {
        println!("Both left and right are ints. left_int={}, right_int={}", left_int, right_int);
        if left_int < right_int {
            println!("Left int smaller than right int, list pair in right order.");
            return CompareResult::RightOrder;
        } else if left_int > right_int {
            println!("Left int larger than right int, list pair in wrong order.");
            return CompareResult::WrongOrder;
        } else {
            println!("Ints are the same. Continuing comparing...");
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
        println!("Found start of int!");
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