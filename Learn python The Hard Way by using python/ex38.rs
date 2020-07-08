
fn main() {
    let ten_things = "Apples Oranges Crows Telephone Light Sugar";
    let mut vec: Vec<&str> = ten_things.split(' ').collect();
    println!("Wait there are not 10 things in that list. Let's fix that.");
    let mut more_stuff:Vec<&str> = Vec::new();
    more_stuff.push("Day");
    more_stuff.push("Night");
    more_stuff.push("Song");
    more_stuff.push("Frisbee");
    more_stuff.push("Corn");

    while vec.len() != 10{
        let next_one = more_stuff.pop().unwrap();
        println!("Adding: {:?}",next_one);
        vec.push(next_one);
        println!("There are {} items now", vec.len());
    }
    println!("There we go: ");
    for i in &vec {
        print!("{} ", i);
    }
}
fn main() {
    let ten_things = "Apples Oranges Crows Telephone Light Sugar";
    let mut vec: Vec<&str> = ten_things.split(' ').collect();
    println!("Wait there are not 10 things in that list. Let's fix that.");
    let mut more_stuff:Vec<&str> = Vec::new();
    more_stuff.push("Day");
    more_stuff.push("Night");
    more_stuff.push("Song");
    more_stuff.push("Frisbee");
    more_stuff.push("Corn");

    while vec.len() != 10{
        let next_one = more_stuff.pop().unwrap();
        println!("Adding: {:?}",next_one);
        vec.push(next_one);
        println!("There are {} items now", vec.len());
    }
    println!("There we go: ");
    for i in &vec {
        print!("{} ", i);
    }
}