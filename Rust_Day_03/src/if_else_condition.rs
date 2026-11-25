fn main(){
    println!("Hello let's begin coding");

    // Rust if else condition

    if 7 > 5{
        println!("Yes seven is greater than five");
    }

    let s = 34;
    let y = 45;

    if s > y {
        println!("Hello world");
    } else {
        println!("Go to hell");
    }

    // mutliple if else statements

    let score = 90;

    if score >= 90 {
        println!("Grade A");
    } else if score >= 80{
        println!("Grade B");
    } else if score >= 70{
        println!("Grade C");
    } else if score >= 60{
        println!("Grade D");
    } else if score >= 50 {
        println!("Grade E");
    } else {
        println!("Fail");
    }

    let time = 20;

    let greeting = if time < 18 {
        "Good day"
    } else {
        "Good evening"
    };

    println!("Hello, {}", greeting);
    
}