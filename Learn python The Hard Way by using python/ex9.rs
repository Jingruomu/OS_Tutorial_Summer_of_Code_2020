fn main() {
    let mut numbers = [0,0,0,0];
    let mut name:[char;4] = ['a','a','a','a'];
    // Seems rust doesn't support only initialize the first value of the array like in C. 
    // Found in stackoverflow 
    println!("numbers: {} {} {} {}",
             numbers[0], numbers[1], numbers[2], numbers[3]);
    println!("name each: {} {} {} {}",
             name[0], name[1], name[2], name[3]);
    println!("name: {:?}", name);

    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    numbers[3] = 4;

    name[0] = 'Z';
    name[1] = 'e';
    name[2] = 'd';
    name[3] = '\0';

    println!("numbers: {} {} {} {}",
             numbers[0], numbers[1], numbers[2], numbers[3]);
    println!("name each: {} {} {} {}",
             name[0], name[1], name[2], name[3]);
    println!("name: {:?}", name);

    let name2 = "zed".to_string();
    println!("{}",name2);

    for i in name2.chars() {
        print!("{},", i);
    }

    println!("{:?}",name2.chars().nth(2).unwrap())
}