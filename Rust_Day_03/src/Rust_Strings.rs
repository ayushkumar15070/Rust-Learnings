fn main(){
    println!("Hello world");

    let greeting:&str = "Ayush";
    println!("Hello, {} ", greeting);

    // we can create a string using this thing as well

    let string = "Hello world one more time".to_string();
    println!("{}",string);

    // we can create string using this one as well
    let anotherstring = String::from("Hello one more time to the world");
    println!("Hello: {}", anotherstring);


    // Strings as mutable if they are declared with the mut keyword

    let mut string2 = String::from("Hello world, one more time as i am learning that's why i am using this much of the hello world thank you");
    string2.push_str("Ayush is the world's number one coder and he desires to be a legend in the upcoming 10 years and you know what's more interesting he don't know what he has to do with his life in those next 10 years but still dreams to become the legend and i think he will become a legend");

    println!("The updated string is: {} ", string2);


    let mut word = String::from("Hello world");
    word.push('D');
    println!("{}", word);


    // we use the format function or keyword to concatenate the strings

    let str1 = String::from("Hello world, one time");
    let str2 = String::from("Hello world, two time");
    let str3 = String::from("Hello world, three time");

    let result = format!("{} {} {}", str1, str2, str3);
    println!("The string after concatenation is {}", result);


    // we can also concatenate strings by using this thing as well 

    let firstname = String::from("Ayush");
    let middlename = String::from("Kumar");
    let lastname = String::from("Vishwakarma");
    let result = firstname + " " + &middlename + " " + &lastname;

    println!("Your name is: {}", result);

    let name = String::from("Ayush is the best coder in the world");

    println!("The string length is: {}", name.len());
}