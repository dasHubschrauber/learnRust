use std::io;

fn main() {
    println!("Welcome to Counter!");

    let mut count: isize = 0;
    let online = true;
    let mut old_count = 0;

    while online {

        println!("Your counter is currently set at {}.", count);
        println!("Would you like to increase (I) or decrease (D) you counter? If you want to exit, use E");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading the value.");

        let t = String::from(choice);

        match t.as_str() {
            "I\r\n" => println!("By how much would you like to increase the counter?"),
            "D\r\n" => println!("By how much would you like to decrease the counter?"),
            "E\r\n" => break,
            _ => println!("t = {:?}", t),
        }

        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Error reading the increase amount");

        let c = amount.trim().parse().expect("That is not a valid number");

        old_count = count;

        if t.eq("I\r\n") {
            count = increase(count, c);
            println!("You increased the counter from {} to {}", old_count, count);
        }
        else if t.eq("D\r\n") {
            count = decrease(count, c);
            println!("You decreased the counter from {} to {}", old_count, count);
        }
    }
}

fn increase(num: isize, inc: isize) -> isize {
    num + inc
}


fn decrease(num: isize, inc: isize) -> isize {
    num - inc
}