use std::fs;
use std::ops::Index;

fn main() {
    println!("Hello, world!");
    let file = "input";
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file"); //scissor

    let mut point :i32 = 0;
    for line in contents.lines() {
        let me = line.to_owned().chars().nth(2).unwrap();
        let op = line.to_owned().chars().next().unwrap();
        println!("{}",line);
        println!("{}",me);

        match me {
            'X' => point += 0,
            'Y' => point += 3,
            'Z' => point += 6,
            _ => point += 0
        }

        match line {
            // Rock Paper Sciccor
            "A Y" | "B X" | "C Z" => point += 1, // Rock
            "A Z" | "B Y" | "C X" => point += 2, // Paper
            "A X" | "B Z" | "C Y" => point += 3, // Scissor
            _ => break

        }
        println!("{}", point);

    }


}