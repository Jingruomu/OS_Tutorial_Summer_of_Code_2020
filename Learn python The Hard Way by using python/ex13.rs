use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("ERROR: You need one argument.");
    }
    else {
        let mut i = 0;
        while i < args[1].len() {
            let letter= args[1].chars().nth(i).unwrap();
            match letter {
                'a' => println!("{} 'A'", i),
                'A' => println!("{} 'A'", i),
                'e' => println!("{} 'E'", i),
                'E' => println!("{} 'E'", i),
                'i' => println!("{} 'I'", i),
                'I' => println!("{} 'I'", i),
                'o' => println!("{} 'O'", i),
                'O' => println!("{} 'O'", i),
                'u' => println!("{} 'u'", i),
                'U' => println!("{} 'U'", i),
                'y' => println!("{} 'y'", i),
                'Y' => println!("{} 'Y'", i),
                _ => println!("{}: {} is not a vowel.", i, letter),
            }
            i += 1;
        }
    }
}
