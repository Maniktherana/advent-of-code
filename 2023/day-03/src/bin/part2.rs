use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Part {
    value: i32,
    pair: i32,
    has_star: bool,
}

fn part2(input: &str) -> i32 {
    calculate_sum(input)
}

fn is_symbol(c: char) -> bool {
    c == '*'
}

fn is_valid_index(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    row < max_row && col < max_col && row >= 0 && col >= 0
}

fn calculate_sum(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut nums: Vec<Part> = Vec::new();
    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (row_idx, row) in input.lines().enumerate() {
        for mat in re.find_iter(row) {
            let start_idx = mat.start();
            let end_idx = mat.end();
            let parsed_num = mat.as_str().parse::<i32>().unwrap_or(0);

            let mut gear_ratio = Part {
                value: parsed_num,
                has_star: false,
                pair: 0,
            };

            for col_idx in start_idx..end_idx {
                for (dx, dy) in &directions {
                    let new_row = row_idx as i32 + dx;
                    let new_col = col_idx as i32 + dy;

                    if is_valid_index(new_row, new_col, board.len() as i32, board[0].len() as i32) {
                        let adjacent_char: char = board[new_row as usize][new_col as usize];
                        if is_symbol(adjacent_char) {
                            gear_ratio.has_star = true;

                            for (dx, dy) in &directions {
                                let star_new_row = new_row as i32 + dx;
                                let star_new_col = new_col as i32 + dy;
                                let new_row_string =
                                    board[star_new_row as usize].iter().collect::<String>();

                                for mat in re.find_iter(new_row_string.as_str()) {
                                    let start_idx = mat.start();
                                    let end_idx = mat.end();
                                    let new_parsed_num = mat.as_str().parse::<i32>().unwrap_or(0);

                                    if start_idx <= (star_new_col as usize)
                                        && (star_new_col as usize) < end_idx
                                    {
                                        gear_ratio.pair = new_parsed_num;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if gear_ratio.has_star && gear_ratio.value != gear_ratio.pair {
                nums.push(gear_ratio);
            }
        }
    }
    let mut sum = 0;
    for num in nums {
        sum += num.value * num.pair;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("./test.txt");
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
