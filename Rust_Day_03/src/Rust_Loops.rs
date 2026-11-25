fn main(){
    println!("Hello world");

    // Loops
    // loop {
    //     println!("Hello world")
    // }

    let mut count = 3;

    loop {
        println!("Hello world");

        if count == 3 {
            break;
        }

        count = count + 1;
    }


    let mut anothercount = 4;
    
    let result = loop {
        println!("Hello world");

        if count == 3{
            break anothercount;
        }

        anothercount = anothercount + 1;
    };

    println!("The loop stoped in: {}", result);
}