use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut res = 0;

    let regex_pattern = r"(?=(one|two|three|four|five|six|seven|eight|nine))|(\d)";
    let re = Regex::new(regex_pattern).unwrap();

    for line in input.lines() {
        let mut matches = Vec::new();
        for mat in re.find_iter(line) {
            if let Some(digit_match) = mat.as_str().parse::<i32>().ok() {
                matches.push(digit_match);
            } else {
                match mat.as_str() {
                    "one" => matches.push(1),
                    "two" => matches.push(2),
                    "three" => matches.push(3),
                    "four" => matches.push(4),
                    "five" => matches.push(5),
                    "six" => matches.push(6),
                    "seven" => matches.push(7),
                    "eight" => matches.push(8),
                    "nine" => matches.push(9),
                    _ => continue,
                }
            }
        }
        let first = matches.first().unwrap();
        let last = matches.last().unwrap();
        let num = first * 10 + last;
        res += num;
        println!("{line} {:?}", matches);
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