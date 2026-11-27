fn main() {
    println!("Hello, world!");
    // Rust Tuples
    let person = ("Ayush", 20, true);
    println!("Hello world");
    println!("The person name is :{}", person.0);
    println!("The person's age is :{}", person.1);
    println!("The person is employees or not: {}", person.2);

    // we can destructure a tuple like this also 

    let anotherperson = ("Ayush", 20, true);
    let (name, age, employee) = anotherperson;

    println!("The name of another person is: {}", name);
    println!("The age of another person is: {}", age);
    println!("The another person is employeed or not: {}", employee);


    fn get_user() -> (String, i32){
        (String::from("Ayush"), 20)
    }

    let onemoreperson  = get_user();
    println!("The name of one more person is: {}", onemoreperson.0);
    println!("The age of one more person is: {}", onemoreperson.1);
}


