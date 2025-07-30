use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: {} [action]", args[0]);
        println!("Available actions: loop, while, panic, dfs, bfs, readfile");
        return;
    }
    let mut what_you_want_to_run: String = String::new();
    let actions: Vec<&str> = vec!["loop", "while", "panic", "dfs", "bfs", "readfile"];
    println!("Available actions: {:?}", actions);

    println!("Please enter the action you want to run (loop/while):");
    io::stdin()
        .read_line(&mut what_you_want_to_run)
        .expect("Failed to read line");
    println!("You choosed: {}", what_you_want_to_run.trim());

    match what_you_want_to_run.trim().to_lowercase().as_str() {
        //.to_lowercase().as_str() for case-insensitive matching
        "loop" => print_loop(),
        "while" => print_while(),
        "panic" => panic_function(),
        "dfs" => dfs_bfs_example(),
        "bfs" => dfs_bfs_example(),
        "readfile" => read_file(),
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

fn panic_function() {
    let numbers = vec![1, 2, -3];
    process_numbers(&numbers);
}
fn process_numbers(numbers: &[i32]) {
    for &number in numbers {
        if number < 0 {
            // it will panic if a negative number is found
            //and use to block execution of code but not recommended in production code
            panic!("Negative number found: {}", number);
        }
        println!("Processing number: {}", number);
    }
}

type Graph = HashMap<String, Vec<String>>;
fn build_graph() -> Graph {
    let mut graph: Graph = HashMap::new();
    graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    graph.insert("B".to_string(), vec!["D".to_string()]);
    graph.insert("C".to_string(), vec!["D".to_string(), "E".to_string()]);
    graph.insert("D".to_string(), vec!["F".to_string()]);
    graph.insert("E".to_string(), vec!["F".to_string()]);
    graph.insert("F".to_string(), vec![]);
    graph
}

fn dfs(graph: &Graph, start: &str, visited: &mut HashSet<String>) {
    if visited.contains(start) {
        return;
    }
    visited.insert(start.to_string());
    println!("Visiting: {}", start);
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}

fn bfs(graph: &Graph, start: &str) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.to_string());
    queue.push_back(start.to_string());

    while let Some(node) = queue.pop_front() {
        println!("Visiting: {}", node);
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.to_string());
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    }
}

fn dfs_bfs_example() {
    let graph: HashMap<String, Vec<String>> = build_graph();
    let start_node: &'static str = "A";

    println!(
        "Depth-First Search (DFS) starting from node '{}':",
        start_node
    );
    let mut visited = HashSet::new();
    dfs(&graph, start_node, &mut visited);

    println!(
        "\nBreadth-First Search (BFS) starting from node '{}':",
        start_node
    );
    bfs(&graph, start_node);
}

fn read_file() {
    let file = match File::open("notfound.txt") {
        Ok(file) => {
            println!("File opened successfully.");
            file
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found, creating a new one.");
                match File::create("notfound.txt") {
                    Ok(mut created_file) => {
                        println!("File created successfully.");
                        // Try to write some initial content to the newly created file
                        match created_file.write_all(b"Welcome to the new file!\nThis is line 2.\n") {
                            Ok(_) => println!("Initial content written to file."),
                            Err(write_err) => match write_err.kind() {
                                io::ErrorKind::PermissionDenied => {
                                    println!("Warning: Permission denied when writing to file. The file was created but may be read-only.");
                                }
                                _ => {
                                    println!("Warning: Could not write initial content: {}", write_err);
                                }
                            }
                        }
                        
                        // Now try to reopen the file for reading
                        match File::open("notfound.txt") {
                            Ok(reopened_file) => {
                                println!("Successfully reopened the newly created file for reading.");
                                reopened_file
                            }
                            Err(reopen_err) => match reopen_err.kind() {
                                io::ErrorKind::PermissionDenied => {
                                    println!("Permission denied: Cannot read the newly created file. Continuing with empty processing...");
                                    return; // Exit gracefully without panicking
                                }
                                _ => {
                                    println!("Error reopening file: {}. Continuing with empty processing...", reopen_err);
                                    return; // Exit gracefully without panicking
                                }
                            }
                        }
                    }
                    Err(create_err) => match create_err.kind() {
                        io::ErrorKind::PermissionDenied => {
                            println!("Permission denied: Cannot create file in this directory. Please check your permissions.");
                            println!("Continuing execution without file operations...");
                            return; // Exit gracefully without panicking
                        }
                        io::ErrorKind::AlreadyExists => {
                            println!("File already exists (race condition). Trying to open again...");
                            match File::open("notfound.txt") {
                                Ok(existing_file) => existing_file,
                                Err(e) => {
                                    println!("Error opening existing file: {}. Continuing without file operations...", e);
                                    return;
                                }
                            }
                        }
                        _ => {
                            println!("Error creating file: {}. Continuing without file operations...", create_err);
                            return; // Exit gracefully without panicking
                        }
                    }
                }
            }
            io::ErrorKind::PermissionDenied => {
                println!("Permission denied: Cannot open file for reading. Please check your permissions.");
                println!("Continuing execution without file operations...");
                return; // Exit gracefully without panicking
            }
            _ => {
                println!("Error opening file: {}. Continuing without file operations...", e);
                return; // Exit gracefully without panicking
            }
        },
    };

    let reader = io::BufReader::new(file);
    println!("Reading file contents:");
    let mut line_count = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => {
                line_count += 1;
                println!("Line {}: {}", line_count, content);
            }
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    println!("Permission denied while reading line {}. Skipping...", line_count + 1);
                }
                _ => {
                    println!("Error reading line {}: {}", line_count + 1, e);
                }
            }
        }
    }
    
    if line_count == 0 {
        println!("File is empty or no content could be read.");
    } else {
        println!("Successfully read {} lines from the file.", line_count);
    }
}
