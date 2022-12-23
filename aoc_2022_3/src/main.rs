use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Hello, world!");
    let file = "input";
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");


    let mut point = 0;
    let mut lines = contents.lines();
    loop {
        let l1 = lines.next().unwrap().to_string();
        let l2 = lines.next().unwrap().to_string();
        let l3 = lines.next().unwrap().to_string();
        let hs1 = HashSet::<char>::from_iter(l1.chars());
        let hs2 = HashSet::<char>::from_iter(l2.chars());
        let hs3 = HashSet::<char>::from_iter(l3.chars());
        let a = hs1.intersection(&hs2).copied().collect::<HashSet<char>>()
            .intersection(&hs3).copied().collect::<HashSet<char>>();
        for l in a {
            let mut value = 0;
            println!("Char: {}", l);
            if l.is_lowercase() {
                value = l.to_digit(36).unwrap() - 9;
            }
            if l.is_uppercase() {
                value = l.to_digit(36).unwrap() + 17;
            }
            println!("Value {}", value);
            point += value;
            println!("Points {}", point);
        }

        if  lines.clone().peekable().peek().is_none() {
            break;
        }
    }
    for line in contents.lines() {
        let len = line.len();
        let middel = len / 2;
        let (c1, c2) = line.split_at(middel);
        let hs1 = HashSet::<char>::from_iter(c1.chars());
        let hs2 = HashSet::<char>::from_iter(c2.chars());
        let a = hs1.intersection(&hs2).map(|i| *i).collect::<Vec<char>>();
        for l in a {
            let mut value = 0;
            println!("Char: {}", l);
            if l.is_lowercase() {
                value = l.to_digit(36).unwrap() - 9;
            }
            if l.is_uppercase() {
                value = l.to_digit(36).unwrap() + 17;
            }
            println!("Value {}", value);
            point += value;
            println!("Points {}", point);
        }
    }
}
