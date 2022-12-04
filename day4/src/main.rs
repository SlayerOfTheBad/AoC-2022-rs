fn part1(input: &str) {
    let result = input
        .lines()
        .map(|x| x.split(','))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(x, y)| {
            (
                (
                    x.split('-').next().unwrap().parse::<u32>().unwrap(),
                    x.split('-').last().unwrap().parse::<u32>().unwrap(),
                ),
                (
                    y.split('-').next().unwrap().parse::<u32>().unwrap(),
                    y.split('-').last().unwrap().parse::<u32>().unwrap(),
                ),
            )
        })
        .map(|((x1, x2), (y1, y2))| (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2))
        .filter(|x| x.to_owned())
        .collect::<Vec<bool>>()
        .len()
    ;
    println!("{}", result);
}

fn part2(input: &str) {
    let result = input
        .lines()
        .map(|x| x.split(','))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(x, y)| {
            (
                (
                    x.split('-').next().unwrap().parse::<u32>().unwrap(),
                    x.split('-').last().unwrap().parse::<u32>().unwrap(),
                ),
                (
                    y.split('-').next().unwrap().parse::<u32>().unwrap(),
                    y.split('-').last().unwrap().parse::<u32>().unwrap(),
                ),
            )
        })
        .map(|((x1, x2), (y1, y2))| (x1 <= y1 && x2 >= y1) || (y1 <= x1 && y2 >= x1))
        .filter(|x| x.to_owned())
        .collect::<Vec<bool>>()
        .len()
    ;
    println!("{}", result);
}

fn main() {
    let input = include_str!("../input1.txt");
    part1(input);
    part2(input);
}
