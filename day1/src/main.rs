fn main() {
    let mut elves = include_str!("../input1.txt")
            .split("\n\n")
            .map(|x| x.split("\n").filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()))
            .map(|x| x.reduce(|accum, x| accum + x).unwrap())
            .collect::<Vec<u32>>();
    elves.sort_by(|a, b| b.cmp(a));
    println!(
        "{}",
        elves[0]
    );

    elves.truncate(3);
    println!(
        "{}",
        elves.into_iter().reduce(|accum, x| accum + x).unwrap()
    );
}
