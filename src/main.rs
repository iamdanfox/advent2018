use std::fs;

fn main() {
    println!("Hello, world!");
}

#[test]
fn day_1() {
    let result = fs::read_to_string("day1.txt").unwrap();
    let lines = result.lines();

    let mut freq: isize = 0;
    for line in lines {
        let change: isize = line.parse().expect(&format!("trying to parse '{}'", line));
        freq = freq + change;
    }
    println!("finished {}", freq)
}
