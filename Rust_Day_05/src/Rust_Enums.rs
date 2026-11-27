use core::panic::PanicMessage;

fn main(){
    println!("Hello world");

    enum Direcrtions {
        Up, 
        Down, 
        Right,
        Left
    }
    
    let mydirection = Direcrtions::Up;
    println!("Going up");

    let myanotherfirection  = Direcrtions::Left;

    match myanotherfirection {
        Direcrtions::Up => println!("Going Up"),
        Direcrtions::Down => println!("Going Down"),
        Direcrtions::Left => println!("Going Left"),
        Direcrtions::Right => println!("Going Right"),
    }

    enum Loginstatus {
        Success(String),
        Error(String)
    }

    let result1 = Loginstatus::Success(String::from("Welcome back"));
    let result2 = Loginstatus::Error(String::from("Something is wrong"));

    match result1 {
        Loginstatus::Success(message) => println!("Success : {}", message),
        Loginstatus::Error(message) => println!("Error : {}", message),
    }

    match result2 {
        Loginstatus::Error(errormessage) => println!("Error: {}", errormessage),
        Loginstatus::Success(successmessage) => println!("Success: {}", successmessage),
     }
}