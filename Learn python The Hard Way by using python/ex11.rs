use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut x = 0;
    while x < args.len(){
        println!("{}",args[x]);
        x = x+1;
    }

    let states = ["California", "Oregon",
        "Washington", "Texas"];
    let mut y = 0;
    while y < states.len(){
        println!("{}",states[y]);
        y = y+1;
    }


}
