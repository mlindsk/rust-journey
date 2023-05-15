// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    // TODO: Implement fmt Debug and print method
    name: String,
    age: u8,
}


fn main() {

    // let name = String::new(); // New instance with no content
    // let name = String::from("Mads"); // Initiate with content
    let name = "Mads".to_string();
    let age = 35;
    let mads = Person {name, age };


    
    
        
}
