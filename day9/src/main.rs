use std::collections::HashSet;

fn distance((a, b): (i32, i32), (x, y): (i32, i32)) -> i32 {
    (x - a).abs().max((y - b).abs())
}

fn relative_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    (head.0 - tail.0, head.1 - tail.1)
}

fn part1(input: &str) -> HashSet<(i32, i32)> {
    let mut points = HashSet::new();

    let mut head = (0,0);
    let mut tail = (0,0);
    points.insert(tail);

    for l in input.lines() {
        let dir = l.chars().nth(0).unwrap();
        let count = l.split(" ").nth(1).unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            let old_head = head;
            match dir {
                'U' => head = (head.0, head.1+1),
                'D' => head = (head.0, head.1-1),
                'R' => head = (head.0-1, head.1),
                'L' => head = (head.0+1, head.1),
                _ => panic!("wrong input"),
            }

            if distance(head, tail) > 1 {
                tail = old_head;
                points.insert(tail);
            }
        }
    }

    points
}

fn part2(input: &str) -> HashSet<(i32, i32)> {
    let mut points = HashSet::new();

    let mut rope = vec![(0i32,0i32); 10];
    points.insert(rope.last().unwrap().to_owned());

    for l in input.lines() {
        let dir = l.chars().nth(0).unwrap();
        let count = l.split(" ").nth(1).unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            match dir {
                'U' => rope[0] = (rope[0].0, rope[0].1+1),
                'D' => rope[0] = (rope[0].0, rope[0].1-1),
                'R' => rope[0] = (rope[0].0-1, rope[0].1),
                'L' => rope[0] = (rope[0].0+1, rope[0].1),
                _ => panic!("wrong input"),
            }

            for k in 1..rope.len() {
                if distance(rope[k], rope[k-1]) <= 1 {
                    continue;
                }

                rope[k] = match relative_position(rope[k-1], rope[k]) {
                    (0, 2) => (rope[k].0, rope[k].1+1),
                    (0, -2) => (rope[k].0, rope[k].1-1),
                    (2, 0) => (rope[k].0+1, rope[k].1),
                    (-2, 0) => (rope[k].0-1, rope[k].1),

                    (1, 2) => (rope[k].0+1, rope[k].1+1),
                    (1, -2) => (rope[k].0+1, rope[k].1-1),
                    (2, 1) => (rope[k].0+1, rope[k].1+1),
                    (-2, 1) => (rope[k].0-1, rope[k].1+1),

                    (-1, 2) => (rope[k].0-1, rope[k].1+1),
                    (-1, -2) => (rope[k].0-1, rope[k].1-1),
                    (2, -1) => (rope[k].0+1, rope[k].1-1),
                    (-2, -1) => (rope[k].0-1, rope[k].1-1),

                    (2, 2) => (rope[k].0+1, rope[k].1+1),
                    (2, -2) => (rope[k].0+1, rope[k].1-1),
                    (-2, 2) => (rope[k].0-1, rope[k].1+1),
                    (-2, -2) => (rope[k].0-1, rope[k].1-1),
                    k => panic!("Rope moved too quickly {:?}! :c", k),
                }
            }

            points.insert(rope.last().unwrap().to_owned());
        }
    }

    points
}

fn main() {
    let input = include_str!("../input1.txt");
    println!("{:#?}", part1(input).len());
    println!("{:#?}", part2(input).len());
}
