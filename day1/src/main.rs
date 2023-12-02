fn main() {
    let input = include_str!("../input.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let first = line.chars().filter(|c| c.is_digit(10)).next().unwrap();
        let last = line.chars().filter(|c| c.is_digit(10)).next_back().unwrap();
        let num = format!("{}{}", first, last);
        let n = num.parse::<i32>().unwrap();
        sum += n;
    }

    dbg!(sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits: Vec<i32> = vec![];

        let mut chars = line.chars().peekable();
        while let Some(_) = chars.peek() {
            let s: String = chars.clone().collect();

            if s.starts_with("one") {
                digits.push(1)
            }
            if s.starts_with("two") {
                digits.push(2)
            }
            if s.starts_with("three") {
                digits.push(3)
            }
            if s.starts_with("four") {
                digits.push(4)
            }
            if s.starts_with("five") {
                digits.push(5)
            }
            if s.starts_with("six") {
                digits.push(6)
            }
            if s.starts_with("seven") {
                digits.push(7)
            }
            if s.starts_with("eight") {
                digits.push(8)
            }
            if s.starts_with("nine") {
                digits.push(9)
            }

            let c = chars.next().unwrap();
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as i32);
            }
        }

        let first = digits.iter().next().unwrap();
        let last = digits.iter().next_back().unwrap();
        let num = format!("{}{}", first, last);
        let n = num.parse::<i32>().unwrap();
        sum += n;
    }
    dbg!(sum);
}
