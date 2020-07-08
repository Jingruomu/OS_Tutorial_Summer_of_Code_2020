use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("You only have one argument. You suck.")
    }else if args.len()>1 && args.len()<4 {
        println!("Here's your arguments:");
        for temp in args.iter() {
            print!("{} ", temp);
        }
    }
    else {
        println!("You have too many arguments. You suck.");
    }
    //在Rust中，命令行输入参数时第一个参数是默认的


}
