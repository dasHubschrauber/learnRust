fn main() {
    
    let s = String::from("banana stand");

    let size = calculate_length(&s);

    println!("The size of {} is {}", s, size);

}

fn calculate_length(s1: &String) -> usize {

    s1.len()
}
