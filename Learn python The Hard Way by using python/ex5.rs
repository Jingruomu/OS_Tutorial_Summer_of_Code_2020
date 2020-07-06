fn main() {
    let my_name   = "Gautam Dey";
    let my_age    = 35; // Not a lie.
    let my_height = 74; // inches
    let my_weight = 140; // lbs
    let my_eyes   = "Brown";
    let my_teeth  = "White";
    let my_hair   = "Black";

    println!("Let's talk about {}", my_name);
    println!("Actually that's not too heavy.");
    println!("He's got {0} eyes and {1} hair.", my_eyes,my_hair);
    println!("His teeth are usually {} depending on the coffee.", my_teeth);
    println!("If I add {0}, {1}, and {2} I get {3}.", my_age, my_height, my_weight, my_age + my_height + my_weight)
}