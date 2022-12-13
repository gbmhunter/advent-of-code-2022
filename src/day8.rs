use std::fs;

pub fn run() {
    println!("day8");
    let contents = fs::read_to_string("src/day8.txt").unwrap();

    let mut tree_map: Vec<Vec<u8>> = vec![];

    for line in contents.lines() {
        tree_map.push(Vec::<u8>::new());
        let curr_row = tree_map.len() - 1;
        for char in line.chars() {
            let height: u8 = char.to_digit(10).unwrap() as u8;
            tree_map[curr_row].push(height)
        }
    }

    // Now work out which are visible
    let mut num_trees_visible_from_edge: u32 = 0;
    let mut highest_scenic_score: usize = 0;

    for curr_tree_row_num in 0..tree_map.len() {
        for curr_tree_col_num in 0..tree_map[0].len() {
            // For each tree, check all 4 directions to see if it is visible
            // FROM LEFT
            let curr_tree_height = tree_map[curr_tree_row_num][curr_tree_col_num];
            let mut is_visible_left = true;
            let mut viewing_distance_left = 0;
            for compare_tree_col_num in (0..curr_tree_col_num).rev() {
                viewing_distance_left = curr_tree_col_num - compare_tree_col_num;
                if tree_map[curr_tree_row_num][compare_tree_col_num] >= curr_tree_height {
                    // Tree is blocked!
                    is_visible_left = false;
                    break; // Don't need to keep checking from this side now we know it's blocked
                }
            }

            // FROM RIGHT
            let mut is_visible_right = true;
            let mut viewing_distance_right = 0;
            for compare_tree_col_num in curr_tree_col_num + 1..tree_map[0].len() {
                viewing_distance_right = compare_tree_col_num - curr_tree_col_num;
                if tree_map[curr_tree_row_num][compare_tree_col_num] >= curr_tree_height {
                    // Tree is blocked!
                    is_visible_right = false;
                    break; // Don't need to keep checking from this side now we know it's blocked
                }
            }

            // FROM TOP
            let mut is_visible_top = true;
            let mut viewing_distance_top = 0;
            for compare_tree_row_num in (0..curr_tree_row_num).rev() {
                viewing_distance_top = curr_tree_row_num - compare_tree_row_num;
                if tree_map[compare_tree_row_num][curr_tree_col_num] >= curr_tree_height {
                    // Tree is blocked!
                    is_visible_top = false;
                    break; // Don't need to keep checking from this side now we know it's blocked
                }
            }

            // FROM BOTTOM
            let mut is_visible_bottom = true;
            let mut viewing_distance_bottom = 0;
            for compare_tree_row_num in (curr_tree_row_num + 1)..tree_map.len() {
                viewing_distance_bottom = compare_tree_row_num - curr_tree_row_num;
                if tree_map[compare_tree_row_num][curr_tree_col_num] >= curr_tree_height {
                    // Tree is blocked!
                    is_visible_bottom = false;
                    break; // Don't need to keep checking from this side now we know it's blocked
                }
            }

            if is_visible_left || is_visible_right || is_visible_top || is_visible_bottom {
                num_trees_visible_from_edge += 1;
            }
            
            let scenic_score = viewing_distance_top * viewing_distance_bottom * viewing_distance_left * viewing_distance_right;
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    println!("part 1: num_trees_visible={}", num_trees_visible_from_edge);
    println!("part 2: highest_scenic_score={}", highest_scenic_score);
}