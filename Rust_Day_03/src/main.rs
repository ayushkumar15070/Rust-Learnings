fn main() {
    println!("Hello, world!");
    //Boolean Datatype

    let booldatatype:bool = true;
    let booldatatyle2:bool = false;

    println!("Is Car is an amazing machine: {}", booldatatype);
    println!("Is fish is tasty: {}", booldatatyle2);


    // without specifying the type of the boolean Datatype

    let anotherbool = true;
    let anotherbool2 = false;

    println!("This is true: {}", anotherbool);
    println!("This is false: {}", anotherbool2);


    let age = 20;
    let canvote = age >= 18;

    println!("You can vote as you are above 18 years old: {}", canvote);
    
    let is_logged_in = true;

    if is_logged_in {
        println!("Welcome Back!!!");
    } else{
        println!("Get out of this website");
    }
}
