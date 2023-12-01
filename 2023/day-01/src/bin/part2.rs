use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut nums_map: HashMap<&str, i32> = HashMap::new();
    nums_map.insert("one", 1);
    nums_map.insert("two", 2);
    nums_map.insert("three", 3);
    nums_map.insert("four", 4);
    nums_map.insert("five", 5);
    nums_map.insert("six", 6);
    nums_map.insert("seven", 7);
    nums_map.insert("eight", 8);
    nums_map.insert("nine", 9);

    let mut line_arr = Vec::new();

    for line in input.lines() {

        let mut temp = Vec::new();

        for i in 0..line.len() {
            if let Some(ch) = line.chars().nth(i) {
                if ch.is_numeric() {
                    temp.push(ch.to_digit(10).unwrap() as i32);
                }
            }
            for j in 0..line.len() {
                if let Some(substring) = line.get(i..j + 1) {
                    if let Some(&value) = nums_map.get(substring) {
                        temp.push(value);
                    }
                }
            }
        }
        line_arr.push(temp);
    }

    let mut res = 0;

    for num in line_arr {
        let first = num.first().unwrap();
        let last = num.last().unwrap();
        res += first * 10 + last;
    }

    res

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let result = part2("\
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281);
    }
}