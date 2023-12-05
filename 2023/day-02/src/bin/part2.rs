use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_color(captures: Option<regex::Captures>, group_index: usize) -> i32 {
    captures
        .and_then(|c| c.get(group_index))
        .and_then(|m| m.as_str().parse::<i32>().ok())
        .unwrap_or(0)
}

fn part2(input: &str) -> i32 {
    let mut games: Vec<Game> = Vec::new();
    let mut res = 0;

    let game_id_re = Regex::new(r"Game (\d+):").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();

    for line in input.lines() {
        let id = game_id_re
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        // split line by ";"
        let parts = line.split(";").collect::<Vec<&str>>();

        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        // get highest value for each color
        for part in parts {
            let part_blue = parse_color(blue_re.captures(part), 1);
            let part_red = parse_color(red_re.captures(part), 1);
            let part_green = parse_color(green_re.captures(part), 1);

            blue = if part_blue > blue { part_blue } else { blue };
            red = if part_red > red { part_red } else { red };
            green = if part_green > green { part_green } else { green };
        }

        let game = Game {
            id,
            red,
            green,
            blue,
        };

        games.push(game);
    }

    // calculate power set and add to res
    for game in &games {
        let mut power_set = 1;
        power_set *= game.red * game.blue * game.green;
        res += power_set;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
