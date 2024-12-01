use std::io::Read;

pub fn load_inputs(day: &str) -> (Vec<u32>, Vec<u32>) {
    let path_str = format!("./inputs/{}.txt", day);
    let path = std::path::Path::new(&path_str);

    let mut input = String::new();

    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut list_one: Vec<u32> = vec![];
    let mut list_two: Vec<u32> = vec![];

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let first = line.next().unwrap().parse::<u32>().unwrap();
        let second = line.next().unwrap().parse::<u32>().unwrap();
        list_one.push(first);
        list_two.push(second);
    }

    (list_one, list_two)
}
