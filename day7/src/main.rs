#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum FsObject {
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
struct Dir {
    name: String,
    size: Option<u32>,
    contents: Vec<FsObject>,
}

impl Dir {
    fn parse_ls(input: &str, contents: Vec<FsObject>) -> (Vec<FsObject>, &str) {
        let mut contents = contents;
        let mut input = input;
        let next = loop {
            if input.starts_with("$") || input.is_empty() {
                break input;
            }
            let (cur, rest) = match input.split_once("\n") {
                Some(x) => x,
                None => (input, ""),
            };

            input = rest;

            match cur.split_once(" ") {
                Some(("dir", _)) => (),
                Some((size, name)) => contents.push(FsObject::File(File {
                    size: size.parse().unwrap(),
                    name: name.to_string(),
                })),
                None => panic!("Invalid ls output!"),
            }
        };

        (contents, next)
    }

    fn parse(input: &str, dir: Dir) -> (Dir, &str) {
        let mut dir = dir;
        let mut input = input;
        let next = loop {
            if input.is_empty() {
                break input;
            }

            let (cur, rest) = match input.split_once("\n") {
                Some(x) => x,
                None => (input, ""),
            };

            input = rest;

            input = match cur.split_at(4) {
                ("$ cd", " ..") => break input,
                ("$ cd", dir_name) => {
                    let (x, input) = Dir::parse(input, Dir {
                        name: dir_name.to_string(),
                        contents: Vec::new(),
                        size: None, 
                    });
                    dir.contents.push(FsObject::Dir(x));
                    input
                }
                ("$ ls", _) => {
                    let (x, input) = Dir::parse_ls(input, dir.contents);

                    dir.contents = x;
                    input
                },
                _ => panic!("Invalid Command!")
            };
        };


        (dir, next)
    }

    fn assign_dir_size(dir: &mut Dir) -> u32 {
        let mut size = 0;

        for item in dir.contents.iter_mut() {
            match item {
                FsObject::Dir(dir) => size += Dir::assign_dir_size(dir),
                FsObject::File(file) => size += file.size,
            }
        }

        dir.size = Some(size);

        size
    }
}

fn get_directories_which<'a, F>(dir: &'a Dir, condition: &F) -> Vec<&'a Dir>
where
    F: Fn(u32) -> bool
{
    let mut above = Vec::new();

    match dir.size {
        Some(size) if condition(size) => {
            above.push(dir);
        },
        _ => (),
    }

    for item in dir.contents.iter() {
        match item {
            FsObject::Dir(content) => {
                above.extend(get_directories_which(&content, condition));
            },
            _ => ()
        }

    }

    above
}

fn main() {
    let input = include_str!("../input1.txt");
    let (_, input) = input.split_once("\n").unwrap(); 
    let (mut dir, _) = Dir::parse(input, Dir { name: '/'.to_string(), size: None, contents: Vec::new()});
    Dir::assign_dir_size(&mut dir);
    println!("{}", get_directories_which(&dir, &|x| x <= 100000).iter().fold(0, |accum, x| accum + x.size.unwrap()));

    let required_deletion = 30000000 - (70000000 - dir.size.unwrap());
    let mut result2 = get_directories_which(&dir, &|x| x >= required_deletion);
    result2.sort_by(|a, b| a.size.unwrap().cmp(&b.size.unwrap()));
    println!("{}", result2[0].size.unwrap());
}
