fn part1(input: &str) -> i32 {
    input
        .lines()
        .flat_map(|x| match x.split(" ").collect::<Vec<&str>>()[..] {
            ["addx", x, ..] => vec![0, x.parse::<i32>().unwrap()],
            _ => vec![0],
        })
        .zip(1..)
        .fold((0, 1), |(accum, x), (add, cycle)| {
            if cycle % 40 == 20 {
                (accum + (cycle * x), x + add)
            } else {
                (accum, x + add)
            }
        })
        .0
}

fn part2(input: &str) -> String {
    input
        .lines()
        .flat_map(|x| match x.split(" ").collect::<Vec<&str>>()[..] {
            ["addx", x, ..] => vec![0, x.parse::<i32>().unwrap()],
            _ => vec![0],
        })
        .zip(1..)
        .fold((String::new(), 1), |(display, x), (add, cycle)| -> (String, i32) {
            let pixel = if (cycle-1)%40-1 <= x && x <= (cycle-1)%40+1 { "#" } else { "." };
            println!("Cycle: {}, x: {}, pixel: {}", cycle, x, pixel);
            if cycle % 40 == 0 {
                (display + pixel + "\n", x + add)
            } else {
                (display + pixel, x + add)
            }
        })
        .0
}

fn main() {
    let input = include_str!("../input1.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
