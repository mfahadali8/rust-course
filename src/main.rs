use std::io;
fn main() {
    let mut what_you_want_to_run: String = String::new();
    let actions: Vec<&str> = vec!["loop", "while"];
    println!("Available actions: {:?}", actions);
    println!("Please enter the action you want to run (loop/while):");
    io::stdin()
        .read_line(&mut what_you_want_to_run)
        .expect("Failed to read line");
    println!("You choosed: {}", what_you_want_to_run.trim());
    match what_you_want_to_run.trim() {
        "loop" => print_loop(),
        "while" => print_while(),
        _ => println!("Unknown command!"),
    }
}

fn print_loop() {
    for i in 1..=5 {
        println!("Counting down: {}", i);
    }
    for i in (1..=5).rev() {
        println!("Counting up: {}", i);
    }
    let mut numbers = vec![1, 2, 3, 4, 5];
    for number in &mut numbers {
        println!("Actual Number {} ", number);
        *number *= 2; // Double each number
        println!("After double the {} ", number);
    }
}
fn print_while() {
    let mut input = String::new();
    while input.trim() != "exit" {
        input.clear(); // Clear the input string before reading new input
        println!("Please enter a command (type 'exit' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();
        if trimmed_input != "exit" {
            println!("You entered: {}", trimmed_input);
        }
    }

    println!("Goodbye!");
}
