fn main(){
    let my_num = 5;
    let my_double = 5.5;
    let my_bool = true;
    let my_letter = 'D';
    let my_text = "Ayush";

    // We can tell rust what type of value they should be 
    // let mynum: i32 = 5;
    // let mydouble: f64 = 5.5;
    // let myletter:char = 'D';
    // let mybool:bool = true;
    // let mytext:&str = "Hello";

    println!("The number is {}: ", my_num);
    println!("The double is {}: ", my_double);
    println!("The boolean is {}: ", my_bool);
    println!("The letter is {}: ", my_letter);
    println!("The text is {}: ", my_text);

    let temeperature: i32 = -5;
    let age:u32 = 34;
    let price:f64 = 34.66666;
    let character:char = 'D';
    let string:&str = "Ayush";
    let isloggedin:bool = true;

    println!("The temperature is {}: ", temeperature);
    println!("The age is {}: ", age);
    println!("The price is {}: ", price);
    println!("The character is {}: ", character);
    println!("The string is {}: ", string);
    println!("The user is logged in {}: ", isloggedin);


    // Combining the Datatype
    let name:&str = "Ayush";
    let age: i32 = 23;
    let isadmin:bool = true;

    println!("My name is {}: ", name);
    println!("My age is {}: ", age);
    println!("Is he admin or not {}: ", isadmin);
}