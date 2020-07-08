use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    print_arguments(args.len() as i32,&args);
}

fn print_arguments(argc:i32, argv:&Vec<String>) -> () {
    for temp in  0..argc  {
        print_letters(&argv[temp as usize]);
    }
}

fn print_letters(arg:&String) -> () {
    let mut i = 0;
    while i < arg.len() {
        let ch = arg.chars().nth(i).unwrap();
        println!("{} == {}", ch, ch as i32);
        i += 1;
    }
}
