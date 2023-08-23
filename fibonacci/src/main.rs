use std::io;

fn main() {
    println!("Enter the number you want to discover the fibonacci of?");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Error reading the number.");

    let num: i32 = value.trim().parse().expect("Not a valid number.");
/*
    for iter in 0..20 {
        let res = fib(iter);
        println!("The fibonacci of {} is {}", iter, res);
    }
*/
    let result = fib(num);

    println!("The fibonacci of {} is {}", num, result);
    
}

fn fib(number: i32) -> i32 {

    if number == 0 {
        return 0;
    }
    else if number == 1 {
        return 1;
    }
    else {
        return fib(number-1) + fib(number-2);
    }
}
