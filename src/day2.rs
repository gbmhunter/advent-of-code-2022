use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("day2");

    let selected_shape_scores = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let outcomes = HashMap::from([
        (("A", "X"), "D"),
        (("A", "Y"), "W"),
        (("A", "Z"), "L"),
        (("B", "X"), "L"),
        (("B", "Y"), "D"),
        (("B", "Z"), "W"),
        (("C", "X"), "W"),
        (("C", "Y"), "L"),
        (("C", "Z"), "D"),
    ]);

    let outcome_scores = HashMap::from([("L", 0), ("D", 3), ("W", 6)]);

    let contents = fs::read_to_string("src/day2.txt").unwrap();
    let lines = contents.lines();
    let mut total_score = 0;
    for line in lines.clone() {
        let opponent_choice = &line[0..1];
        let your_choice = &line[2..3];
        // println!("opponent_choice={:?}, your_choice={:?}", opponent_choice, your_choice);
        let selected_shape_score = selected_shape_scores.get(your_choice).unwrap();
        // println!("selected_shape_score={:?}", selected_shape_score);
        let outcome = outcomes.get(&(opponent_choice, your_choice)).unwrap();
        let outcome_score = outcome_scores.get(outcome).unwrap();
        // println!("{:?}", outcome_score);
        total_score += selected_shape_score + outcome_score;
    }
    println!("part 1: total_score={:?}", total_score);

    // Part 2
    let input_to_outcome = HashMap::from([("X", "L"), ("Y", "D"), ("Z", "W")]);
    total_score = 0;
    for line in lines.clone() {
        let opponent_choice = &line[0..1];
        let your_input = &line[2..3];
        let your_outcome = input_to_outcome.get(your_input).unwrap();
        for (inputs, outcome) in &outcomes {
            if opponent_choice == inputs.0 && your_outcome == outcome {
                let your_required_choice = inputs.1;
                total_score += selected_shape_scores.get(your_required_choice).unwrap();
                total_score += outcome_scores.get(outcome).unwrap();
            };
        }
    }
    println!("part 2: total_score={:?}", total_score);
}
