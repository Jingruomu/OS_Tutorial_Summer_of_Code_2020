use std::mem;
fn main() {
    let ages = [23, 42, 12, 89, 2];
    let names = ["Alan", "Frank", "Mary",
        "John", "Lisa"];
    let count =  mem::size_of_val(&ages) / mem::size_of::<i32>();
    let mut i = 0;
    while i < count {
        println!("{} has {} years alive.", names[i], ages[i]);
        i += 1;
    }
    println!("-----");
    let ages_ptr: *const i32 = ages.as_ptr();
    let names_ptr: *const &str = names.as_ptr();
    let mut i = 0;
    while i < count {
        // 不知道怎么把pointers 变成 arrays
        unsafe {
            println!("{} is {} years old.", *names_ptr.offset(i as isize),
                     *ages_ptr.offset(i as isize) );
        }
        i += 1;
    }
}