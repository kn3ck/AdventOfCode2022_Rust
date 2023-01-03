use std::fs;
use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    let file = "input";
    let contents = fs::read_to_string(file).unwrap();

    /*
    Going over instructions
     */
    let mut stack_string:String = String::new();
    let mut move_switch = false;
    let mut move_list: Vec<Move> = vec![];
    for line in contents.lines() {
        // println!("{line}");
        if line.is_empty()  {
            move_switch = true;
            continue;
        }
        if !move_switch {
            stack_string = line.to_owned() + "\n" + &stack_string;
        } else {
            move_list.push(move_from_cmd(line.to_owned()));
        }

    }
    println!("{}", stack_string);
    let mut ship = parse_stack(stack_string);
    for m in move_list {
        crane_move(&mut ship, &m);
    }
    /* Get the top crate for each stack */
    for stack in ship {
        println!("{:?}", stack[stack.len()-1]);
    }

}

fn crane_move(ship: &mut [Vec<char>; 9], m: &Move) {
    println!("{:?}", ship[m.from - 1]);
    println!("{:?}", ship[m.to - 1]);
    println!("{:?}", m);
    let from_copy = ship[m.from - 1].clone();
    let (remaining, moved) = from_copy.split_at(&ship[m.from - 1].len() - m.amount);
    ship[m.from - 1] = remaining.to_owned().to_vec();

    /* PART 1
    // let mut moved: Vec<char> = moved.to_vec().into_iter().rev().collect::<Vec<char>>();
    array[m.to - 1].append(&mut moved);
    */
    /* PART 2 */
    ship[m.to - 1].append(moved.to_vec().as_mut());
    /* PART 2 END */
    println!("{:?}", ship[m.from - 1]);
    println!("{:?}", ship[m.to - 1]);
    println!("--------------------");
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize
}

fn move_from_cmd(cmd: String) -> Move {
    let cmd_part: Vec<&str> = cmd.split_terminator(" ").collect();
    Move {
        from: cmd_part[3].parse().unwrap(),
        to: cmd_part[5].parse().unwrap(),
        amount: cmd_part[1].parse().unwrap(),
    }
}

fn parse_stack(stack: String) -> [Vec<char>; 9] {
    let mut ship: [Vec<char>; 9] = Default::default();
    let mut stack_iter = stack.lines();
    stack_iter.next(); // Skip first line with stack numbers;
    for line in stack_iter {
        let mut empty_count = 0;
        let mut stack_index = 0;
        let parts = line.split_terminator(' ');
        for crate_ in parts.clone() {
            /* Handle stacks that do not reach the top level */
            if crate_.is_empty() {
                empty_count += 1;
                if empty_count == 4 {
                    stack_index += 1;
                    empty_count = 0;
                }
            } else {
                /* Add create to stack */
                let letter = crate_.chars().nth(1).unwrap();
                let pile: &mut Vec<char> = ship[stack_index].as_mut();
                pile.push(letter);
                stack_index += 1;
            }
        }

    }
    println!("{:?}", ship);
    println!("Stack Transformation complete! -> Ship created");
    ship
}
