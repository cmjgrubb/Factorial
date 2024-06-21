use std::io;

fn main() {
    let mut num: u128 = prompt_user();
    let mut factorial: u128 = num;

    while num > 2 {
        num -= 1;
        factorial *= num;
    }

    println!("The factorial of the number is: {}", factorial);
}

fn prompt_user() -> u128 {
    loop {
        println!("Please enter a positive integer:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u128>() {
            Ok(num) => {
                return num;
            },
            _ => println!("You must enter a positive integer."),
        }
    }
}