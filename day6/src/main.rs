fn the_thing(input: &str, len: usize) {
    let mut buff = String::new();
    let mut count_down = 1;
    for c in input.chars().take(len-1) {
        count_down = match buff.rfind(c) {
            Some(n) if n >= count_down - 1 => n + 2,
            _ => count_down,
        };

        buff.push(c);
    }
    let mut input = input.chars();
    for _ in 0..(len-1) { input.next(); }

    for (i, c) in (len..).zip(input) {
        count_down = match buff.rfind(c) {
            Some(n) if n >= count_down - 1 => n + 2,
            _ => count_down,
        };
        count_down -= 1;
        if count_down <= 0 {
            println!("{}", i);
            return;
        }
        buff.remove(0);
        buff.push(c);
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    the_thing(input, 4);
    the_thing(input, 14);
}
