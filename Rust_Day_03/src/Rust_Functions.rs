fn trialfunction(){
    println!("Hello world");
}

fn greet(name:&str){
    println!("Hello, {} ", name);
}

fn addition(a:i32, b:i32) -> i32{
    return a + b;
}

// we can return the value without using the return function as well 

fn multiplication(a:i32, b:i32) -> i32{
    a * b
}

fn main(){
    trialfunction();
    greet("Ayush");
    println!("The sum of two numbers is: {} ", addition(3, 3));
    println!("The multiplication of two numbers is: {} ", multiplication(3, 3));
}