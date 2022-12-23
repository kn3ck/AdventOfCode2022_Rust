use std::collections::binary_heap::Iter;
use std::fs;

fn main() {
    let file = "input";

    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");
    let mut list = Vec::new();
    let mut tmp_sum = 0;
    for line in contents.lines() {
        if line.is_empty() {
            list.push(tmp_sum);
            println!("Sum: {}", tmp_sum);
            println!("Reset Sum!");
            tmp_sum = 0;
        } else {
            tmp_sum += line.parse::<i32>().unwrap();

        }
    }
    list.sort();
    list.reverse();
    let sum: i32 = list[0..3].iter().sum();
    println!("Highest Sum: {}", sum);
}
