use std::collections::HashMap;
fn main(){

    println!("Hello world");
    // a hashmap is a collection of key value pairs in rust programming language
    let mut capitalcities = HashMap::new();

    capitalcities.insert("India", "Delhi");
    capitalcities.insert("United States of America", "Washington DC");
    capitalcities.insert("Name", "Ayush");

    println!("The inserted values are: {:?}", capitalcities);


    if let Some(city) = capitalcities.get("India"){
        println!("The capital of India is: {}", city);
    } else {
        println!("The India is not in the Hashmap");
    }

    // updating an existing values in the HashMap

    let mut anotherhashmap = HashMap::new();

    anotherhashmap.insert("Ayush", "Kumar");
    anotherhashmap.insert("Ayush", "Vishwakarma");

    println!("The updated value for the same key is: {:?}", anotherhashmap);


    // removing an element from the hashmap

    let mut anotheronehashmap = HashMap::new();

    anotheronehashmap.insert("Ayush", "Kumar");
    anotheronehashmap.insert("Vishal", "Kumar");
    anotheronehashmap.insert("Arush", "Vishwakarma");

    anotheronehashmap.remove("Ayush");
    println!("The values of anotheronehashmap is :{:?}", anotheronehashmap);

    // looping through the HashMap
    let mut anothercapitalcity =HashMap::new();

    anothercapitalcity.insert("India", "Delhi");
    anothercapitalcity.insert("China", "Beijing");
    anothercapitalcity.insert("United States of America","Washington DC");


    for (country, city) in anothercapitalcity{
        println!("The Capital of {} is {}", country, city);
    }
}