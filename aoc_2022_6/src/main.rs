use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Hello, world!");
    let file = "input";
    let contents = fs::read_to_string(file).unwrap();
    let chars: Vec<char> = contents.chars().collect();

    // Part Variables replace a
    let packet_length = 4; // Part 1
    let message_length = 14; // Part 2

    for (pos, _char) in chars.iter().enumerate() {
        let seq = chars[pos..pos + message_length].to_vec();
        println!("{:?}, {}", seq, pos);
        let set: HashSet<char> = HashSet::from_iter(seq.into_iter());
        println!("{:?}", set);
        if set.len() > message_length - 1 {
            /* plus the message length */
            println!("Set is 4 at pos: {}", pos + message_length);
            break;
        }
    }
}
