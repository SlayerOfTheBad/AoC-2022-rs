fn part1(input: &str) -> () {
    let result: u32 = input
        .lines()
        .map(|x| {
            let len = x.chars().collect::<Vec<char>>().len();
            (
                x.chars().take(len / 2).collect::<String>(),
                x.chars().skip(len / 2).collect::<String>(),
            )
        })
        .map(|(x, y)| {
            for val in x.chars() {
                if y.contains(val) {
                    return val;
                }
            }
            panic!("invalid input");
        })
        .map(|x| {
            if x.is_ascii_lowercase() {
                x as u32 - 'a' as u32 + 1
            } else {
                x as u32 - 'A' as u32 + 27
            }
        })
        .sum();

    println!("{}", result);
}

fn part2(input: &str) -> () {
    let mut elves = input.lines().peekable();

    let mut teams: Vec<(&str, &str, &str)> = vec![];

    while elves.peek() != None {
        teams.push((
            elves.next().unwrap(),
            elves.next().unwrap(),
            elves.next().unwrap(),
        ));
    }

    let result: u32 = teams
        .iter()
        .map(|(x, y, z)| {
            for val in x.chars() {
                if y.contains(val) && z.contains(val) {
                    return val;
                }
            }
            panic!("invalid input");
        })
        .map(|x| {
            if x.is_ascii_lowercase() {
                x as u32 - 'a' as u32 + 1
            } else {
                x as u32 - 'A' as u32 + 27
            }
        })
        .sum();

    println!("{}", result);
}

fn main() {
    let input = include_str!("../input1.txt");
    part1(input);
    part2(input);
}
