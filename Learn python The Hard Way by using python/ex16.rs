

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
    height: i32,
    weight : i32,
}

fn person_create(name: &'static str, age:i32, height:i32, weight:i32) -> Box<Person> {
    Box::from(Person { name: name, age: age, height: height, weight: weight })
}

impl Person {
    fn print(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Height: {}", self.height);
        println!("Weight: {}", self.weight);
    }
}

fn main() {
    let mut joe = person_create("Joe Alex", 32 ,64 ,140);
    let mut frank = person_create("Frank Blank", 20, 72, 180);
    //println!("Joe is at memory location {:p}", &*joe as *const _);
    let a = &joe;
    let b = &frank;
    println!("Joe is at memory location {:?}", a as *const Box<Person>);
    joe.print();
    println!("Frank is at memory location {:?}", b as *const Box<Person>);
    frank.print();
    println!("-------");
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    joe.print();

    frank.age += 20;
    frank.weight += 20;
    frank.print();
}