fn main(){
    println!("Hello world");

    for i in 1..6{
        println!("Hello world");
        println!("Number is : {} ", i);
    }


    for i in 1..=6{
        println!("Hello world");
    }

    for i in 1..=10{
        if i == 3{
            continue;
        }
        
        if i == 6 {
            break
        }
        println!("the number is: {} ", i);
    }
}