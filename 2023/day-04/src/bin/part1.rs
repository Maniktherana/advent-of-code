use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<i32>,
    my_nums: HashSet<i32>,
}

fn part1(input: &str) -> i32 {
    let cards = parse_input(input);
    calculate_points(&cards)
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(": ").collect();
            let split_by_pipe: Vec<&str> = split_line.get(1).unwrap_or(&"").split(" | ").collect();

            let winning_nums: HashSet<i32> = split_by_pipe
                .get(0)
                .unwrap_or(&"")
                .split(" ")
                .filter_map(|num| num.parse().ok())
                .collect();

            let my_nums: HashSet<i32> = split_by_pipe
                .get(1)
                .unwrap_or(&"")
                .split(" ")
                .filter_map(|num| num.parse().ok())
                .collect();

            Card  {
                winning_nums,
                my_nums,
            }
        })
        .collect()
}

// takes in &[Card] because we dont want to take ownership of the cards as we are only reading from it
fn calculate_points(cards: &[Card]) -> i32 {
    cards.iter().fold(0, |points, card| {
        let intersected: HashSet<i32> = card
            .winning_nums
            .intersection(&card.my_nums)
            .cloned()
            .collect();
        let matched = intersected.len();

        if matched > 0 {
            points + u32::pow(2, matched as u32 - 1) as i32
        } else {
            points
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("./test.txt");
        let result = part1(input);
        assert_eq!(result, 13);
    }
}
