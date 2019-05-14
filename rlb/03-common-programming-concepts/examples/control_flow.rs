fn main() {
    let number = 6;

    // multiple if-else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // it checks each if expression in turn and executes the first body for
        // which the condition holds true.
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let and if
    let condition = true;
    let number = if condition { 5 } else { 6 }; // type alignment
    println!(
        "number should be {} under condition is {}",
        number, condition
    );
}
