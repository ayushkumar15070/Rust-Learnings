fn main(){
    println!("Hello world");

    let a = String::from("Hello");
    let b = a;

    // println!("The string is: {}", a);
    println!("The string is: {}", b);

    let x = 5;
    let y = x;

    println!("x : {}", x);
    println!("y : {}", y);


    let str1 = String::from("Ayush");
    let str2 = str1.clone();

    println!("a : {}", str1);
    println!("b : {}", str2);
}