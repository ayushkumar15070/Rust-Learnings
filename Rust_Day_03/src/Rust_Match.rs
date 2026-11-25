fn main(){
    println!("Hello world");

    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day")
    }


    // multiple matching at once
    let anotherday = 3;

    match anotherday {
        1 | 2 | 3 | 4 | 5 => println!("Weekdays"),
        6 | 7 => println!("Weekends"),
        _ => println!("Something is wrong")
    }
    
    // Match statements with return value

    let matchvalue = 3;

    let result = match matchvalue{
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thurday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid Day"
    };

    println!("The day is: {}", result);
}

