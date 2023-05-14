mod helpers;

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = helpers::Person { name, age };

    // Print debug struct
    println!("{}", peter.name);
}
