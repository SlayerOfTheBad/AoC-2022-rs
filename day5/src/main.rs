#[derive(Debug)]
struct Ship {
    crates: Vec<Vec<char>>,
}

impl Ship {
    fn parse(cratemap: &str) -> Ship {
        let mut lines = cratemap.lines().rev();
        let stacks = lines
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.parse::<usize>().unwrap())
                }
            })
            .max()
            .unwrap();

        let mut crates = vec![Vec::new(); stacks];

        for line in lines {
            let chars = line
                .char_indices()
                .filter_map(|(idx, c)| if idx % 4 == 1 { Some(c) } else { None })
            ;

            for (idx, c) in (0..).zip(chars) {
                if c != ' ' {
                    crates[idx].push(c);
                }
            }
        }

        Ship { crates }
    }

    fn move_crate(&mut self, from: usize, to: usize, amount: usize) -> () {
        let mut temp = Vec::new();
        for _ in 0..amount {
            temp.push(self.crates[from-1].pop().unwrap());
        }
        for _ in 0..amount {
            self.crates[to-1].push(temp.pop().unwrap());
        }
    }

    fn tops(&self) -> String {
        let mut tops = String::new();
        for vec in &self.crates {
            tops.push(vec.last().unwrap().to_owned());
        }

        tops
    }
}

fn part1(crates: &str, input: &str) {
    let mut ship = Ship::parse(crates);
    for line in input.lines() {
        let mut segments = line.split(' ');
        let times = segments.nth(1).unwrap().parse().unwrap();
        let from = segments.nth(1).unwrap().parse().unwrap();
        let to = segments.nth(1).unwrap().parse().unwrap();
        for _ in 0..times {
            ship.move_crate(from, to, 1);
        }
    }

    println!("{:?}", ship.tops());
}

fn part2(crates: &str, input: &str) {
    let mut ship = Ship::parse(crates);
    for line in input.lines() {
        let mut segments = line.split(' ');
        let amount = segments.nth(1).unwrap().parse().unwrap();
        let from = segments.nth(1).unwrap().parse().unwrap();
        let to = segments.nth(1).unwrap().parse().unwrap();
        ship.move_crate(from, to, amount);
    }

    println!("{:?}", ship.tops());
}

fn main() {
    let mut input = include_str!("../input1.txt").split("\n\n");
    let crates = input.next().unwrap();
    let input = input.next().unwrap();

    part1(crates, input);
    part2(crates, input);
}
