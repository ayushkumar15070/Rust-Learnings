fn main(){
    println!("Hello world");

    enum Direcrtions {
        Up, 
        Down, 
        Right,
        Left
    }
    
    let mydirection1 = Direcrtions::Up;
    let mydirection2 = Direcrtions::Down;
    let mydirection3 = Direcrtions::Right;

    match mydirection1 {
        Direcrtions::Up => println!("Going Up"),
        Direcrtions::Down => println!("Going Down"),
        Direcrtions::Left => println!("Going Left"),
        Direcrtions::Right => println!("Going Right"),
    }
    match mydirection2 {
        Direcrtions::Up => println!("Going Up"),
        Direcrtions::Down => println!("Going Down"),
        Direcrtions::Left => println!("Going Left"),
        Direcrtions::Right => println!("Going Right"),
    }
    match mydirection3 {
        Direcrtions::Up => println!("Going Up"),
        Direcrtions::Down => println!("Going Down"),
        Direcrtions::Left => println!("Going Left"),
        Direcrtions::Right => println!("Going Right"),
    }

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