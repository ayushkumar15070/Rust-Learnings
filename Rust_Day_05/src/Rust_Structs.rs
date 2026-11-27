fn main(){
    println!("Hello world");
    // Rust structs/
    // A struct is short for structure, it is a custom data structure that lets you group related values together

    struct Person{
        name:String,
        age:u32,
        can_vote: bool,
        isbillionaire: bool,
    }

    let user = Person {
        name: String::from("Ayush"),
        age: 35,
        can_vote: true,
        isbillionaire: true,
    };

    println!("The name of the person is: {}", user.name);
    println!("The age of the person is: {}", user.age);
    println!("The person can vote or not: {}", user.can_vote);


    // changing the value 

    let mut anotheruser = Person{
        name: String::from("Ayush Kumar Vishwakarma"),
        age: 20, 
        isbillionaire: true,
        can_vote: true,
    };

    anotheruser.age = 26;

    println!("The name of anotheruser is: {}", anotheruser.name);
    println!("The age of anotheruser is: {}", anotheruser.age);
    println!("{} is a billionaire: {}", anotheruser.name, anotheruser.isbillionaire);
    println!("The person can vote: {}", anotheruser.can_vote);
}