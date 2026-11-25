fn message(){
    let message = "This is the message what i wrote you is that i am going to be the number one programmer in the world and you know what is more crazy is that i can feel that energy that i can be the number one coder in the world who can design and build anything whatever he wants to";
    println!("This is the message you can't ignore right now okay!!");
    println!("This is the original message: {} ", message);
}

fn main(){
    println!("Hello world");
    message();
    println!("This is the example of block scope where we can't access the message variable which is declared inside the message function");
    // println!("This is what we can't access: {} ", message); this is what we can't access as it is the block level scope variable which is declared inside the message only 
    let score = 50;

    if score > 40 {
        let result = "You passed the exam";
        println!("The result is: {} ", result);
    }

    // println!("Result is: {} ", result); // this will cause an error


    let x = 34;
    let y = 56;

    println!("The sum of {} and {} is {}: ", x, y, x + y);


    // this is allowed in this programming language i don't know about other programming languages whether this is allowed to write or not but i know about this one 

    let x = 45;

    {
        let x = 23;
        println!("This is inside the block so we can access it: {} ", x);
    }

    println!("This is outside the block so we can access the original outside value only: {} ", x);
}