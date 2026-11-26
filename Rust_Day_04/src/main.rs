use std::{collections::HashMap, vec};

fn main() {
    println!("Hello, world!");

    // Rust arrays 
    let arrays = ["Apple", "Banana", "Cherry"];
    println!("The last element is: {}", arrays[2]);


    // Vectors

    let mut anotherarray = vec!["Ayush", "Kumar"];
    anotherarray.push("Vishwakarma");
    print!("The value of pushed element is: {}", anotherarray[2]);

    // Tuples

    let candidatedetails = ("Ayush", 20, true);
    println!("The candidate name is: {}", candidatedetails.0);
    println!("The candidate age is: {}", candidatedetails.1);
    println!("The candidate is employeed: {}", candidatedetails.2);


    // Hashmaps

    use std::collections::HashMap;

    let mut capitalcities =HashMap::new();
    capitalcities.insert("France", "Paris");
    capitalcities.insert("India", "Delhi");

    println!("The capital city of India is: {}", capitalcities["India"]);

   
}
