use std::fs;

fn main() {
    println!("Hello, world!");
    let file = "input";
    let contents = fs::read_to_string(file).unwrap();

    let mut count = 0;
    for line in contents.lines() {
        let mut t = line.split_terminator(',');
        let first = t.next().unwrap();
        let second = t.next().unwrap();

        let mut tf = first.split_terminator('-');
        let mut ts = second.split_terminator('-');
        let firstn1 = tf.next().unwrap().parse::<i32>().unwrap();
        let firstn2 = tf.next().unwrap().parse::<i32>().unwrap();
        let secondn1 = ts.next().unwrap().parse::<i32>().unwrap();
        let secondn2 = ts.next().unwrap().parse::<i32>().unwrap();
        println!("{}, {}",firstn1, firstn2);
        println!("{}, {}",secondn1, secondn2);

        // Part 1 Clauses
        if  (firstn1 <= secondn1 && firstn2 >= secondn2) || // Second in First
            (firstn1 >= secondn1 && firstn2 <= secondn2) || // First in Second

            // Part 2 only these two clauses :)
            (firstn1 <= secondn1 && firstn2 <= secondn2 && firstn2 >= secondn1) || // first reaches into second
            (firstn1 >= secondn1 && firstn2 >= secondn2 && secondn2 >= firstn1)// second reaches into first
            {
                println!("{} Operlap! +1", line);
                count += 1;
        }

        println!("--------------------");
    }
    println!("{}", count);
}
