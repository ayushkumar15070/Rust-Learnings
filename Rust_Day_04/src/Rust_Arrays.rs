fn main(){
    println!("Hello world");
    let number = [1, 2, 3, 4, 5];
    println!("The first number is: {}", number[0]);

    for i in number{
        println!("The number is {}", i);
    }

    // changing the number

    let mut number1 = [1, 2, 3, 4, 5];
    number1[3] = 34;
    println!("The updated number is: {}", number1[3]);
    println!("The array length is: {}", number1.len());
    let mut i = 0;

    while i < number1.len(){
        println!("The item is: {}", number1[i]);
        i = i + 1;
    }

    let fruits = ["Apple", "Banana", "Cherry"];
    for i in fruits{
        println!("The fruit is: {}", i);
    }

    // printing the entire array at once the most important thing to remember

    let number2 = [1, 2, 3, 4, 5];
    println!("{:?}", number2);

    // printing just one element from the array

    let number3 = [1, 2, 3, 4, 5];
    println!("{}", number3[3]);


    // we cannot add or remove elemnts from an array

    // but if we want to then we can create the vector to remove or add any element

    let mut array2: Vec<&str> = vec!["Apple", "Banana", "Cherry", "kiwi"];
    array2.push("Anotherfruit");
    println!("{:?}", array2);
    
    let mut count: i32 = 1;
    for i in array2{
        println!("Car {} is {}", count, i );
        count = count + 1;
    }




}

