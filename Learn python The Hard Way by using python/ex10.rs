use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for n in 0..args.len(){
        println!("{}",args[n])
    }
    
    let states = ["California", "Oregon",
        "Washington", "Texas"];
    for n in 0..states.len() {
        println!("{}",states[n])
    }
    
    for (i, item) in args.iter().enumerate() {
        println!("args {}: {}", i, item);
    }

    for (i, item) in states.iter().enumerate() {
        println!("state {}: {}", i, item);
    }

}