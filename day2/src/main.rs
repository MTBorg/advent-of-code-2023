use std::cmp::max;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl From<(i32, &str)> for Game {
    fn from((id, s): (i32, &str)) -> Self {
        let s = s.trim_start_matches(|c| c != ':').trim_start_matches(":");
        let s = s.split(";");
        let sets: Vec<Set> = s.map(|l| Set::from(l)).collect();
        Game { id, sets }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .find(|s| s.red > MAX_RED || s.green > MAX_GREEN || s.blue > MAX_BLUE)
            .is_none()
    }

    fn power(&self) -> i32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        self.sets.iter().for_each(|s| {
            max_red = max(max_red, s.red);
            max_green = max(max_green, s.green);
            max_blue = max(max_blue, s.blue);
        });

        return max_red * max_green * max_blue;
    }
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl From<&str> for Set {
    fn from(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        s.split(",").for_each(|s| {
            let mut words = s.trim().split(" ");
            let num = words.next().unwrap().parse::<i32>().unwrap();
            let color = words.next().unwrap().trim();
            match color {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                unknown => panic!("unknown color {}", unknown),
            }
        });
        Self { red, green, blue }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let games: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(id, line)| Game::from((id as i32 + 1, line)))
        .collect();

    let sum = games
        .iter()
        .filter(|g| g.is_possible())
        .fold(0, |acc, e| acc + e.id);
    dbg!(sum);

    let sum = games.iter().fold(0, |acc, e| acc + e.power());
    dbg!(sum);
}
