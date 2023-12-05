use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Part {
    value: i32,
    is_part: bool,
}

fn part1(input: &str) -> i32 {
    calculate_sum(input)
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn is_valid_index(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    row < max_row && col < max_col && row >= 0 && col >= 0
}

fn calculate_sum(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut nums: Vec<i32> = Vec::new();
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

            let mut part = Part {
                value: parsed_num,
                is_part: false,
            };

            for col_idx in start_idx..end_idx {
                for (dx, dy) in &directions {
                    let new_row = row_idx as i32 + dx;
                    let new_col = col_idx as i32 + dy;

                    if is_valid_index(new_row, new_col, board.len() as i32, board[0].len() as i32) {
                        let adjacent_char: char = board[new_row as usize][new_col as usize];
                        if is_symbol(adjacent_char) {
                            part.is_part = true;
                            break;
                        }
                    }
                }
            }
            if part.is_part {
                nums.push(part.value);
            }
        }
    }
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("./test.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
