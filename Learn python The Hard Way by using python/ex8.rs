use std::mem;

fn main() {
    let areas = [10, 12, 13, 14, 20];
    let name = "Zed";
    println!("The size of an int : {}", mem::size_of::<i32>());
    println!("The size of areas (int[]):{}", mem::size_of_val(&areas));
    println!("The number of ints: {}",
             mem::size_of_val(&areas) / mem::size_of::<i32>());
    println!("The size of a char: {}", mem::size_of::<i8>());
    println!("The size of name (char[]) {}", mem::size_of_val(name));
    println!("The number of chars: {}",
             mem::size_of_val(name) / mem::size_of::<i8>());

}