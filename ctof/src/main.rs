use std::io;

fn main() {

    println!("Enter the amount of degrees in Celsius you want to convert to Fahrenheit:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error retrieving the value");

    let num: i32 = guess.trim().parse().expect("That is not a valid temperature!");

    let result = (num * (9/5)) + 32;
    println!("The result is: {result}");

}
