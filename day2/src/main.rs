enum Play {
    Rock,
    Paper,
    Scissors,
}

fn map_play(x: char) -> Play {
    match x {
        'A' | 'X' => Play::Rock,
        'B' | 'Y' => Play::Paper,
        'C' | 'Z' => Play::Scissors,
        _ => panic!("Invalic input"),
    }
}

fn deduce_move((opponent, win): (char, char)) -> Play {
    match opponent {
        'A' => {
            if win == 'Z' {
                Play::Paper
            } else if win == 'Y' {
                Play::Rock
            } else {
                Play::Scissors
            }
        }
        'B' => {
            if win == 'Z' {
                Play::Scissors
            } else if win == 'Y' {
                Play::Paper
            } else {
                Play::Rock
            }
        }
        'C' => {
            if win == 'Z' {
                Play::Rock
            } else if win == 'Y' {
                Play::Scissors
            } else {
                Play::Paper
            }
        }
        _ => panic!("Invalic input"),
    }
}

fn win(pair: (&Play, &Play)) -> bool {
    match pair {
        (Play::Rock, Play::Paper) => true,
        (Play::Paper, Play::Scissors) => true,
        (Play::Scissors, Play::Rock) => true,
        _ => false,
    }
}

fn part1(input: &str) -> () {
    let result: u32 = input
        .lines()
        .map(|x| {
            (
                map_play(x.chars().nth(0).unwrap()),
                map_play(x.chars().nth(2).unwrap()),
            )
        })
        .map(|(x, y)| {
            let winval = if win((&x, &y)) {
                6
            } else if win((&y, &x)) {
                0
            } else {
                3
            };
            match y {
                Play::Rock => winval + 1,
                Play::Paper => winval + 2,
                Play::Scissors => winval + 3,
            }
        })
        .sum();
    println!("{}", result);
}

fn part2(input: &str) -> () {
    let result: u32 = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (opp, win) = (x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap());
            (map_play(opp), deduce_move((opp, win)))
        })
        .map(|(x, y)| {
            let winval = if win((&x, &y)) {
                6
            } else if win((&y, &x)) {
                0
            } else {
                3
            };
            match y {
                Play::Rock => winval + 1,
                Play::Paper => winval + 2,
                Play::Scissors => winval + 3,
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
