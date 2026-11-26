use std::vec;

fn main(){
    println!("Hello world");
    println!("Hello another world");

    // Vector is resizable array. Unlike the regular arrays, vectors can grow and shrink in size

    let mut fruits = vec!["Apple", "Banana", "Cherry"];
    println!("The first element in the vector array is {}", fruits[0]);

    // changing the Values

    fruits[0] = "Ayush";
    println!("The value of the array after changing the value of that index numbered value: {}", fruits[0]);
    println!("The value of the whole array is {:?}", fruits);

    // adding the elements inside the array

    fruits.push("Anotherfruit");

    println!("The array after inserting the element in the array: {:?}", fruits);

    // removing the elements from the arrays

    fruits.pop();
    println!("The array after removing the element from the array: {:?}", fruits);

    // adding the elements at the specified position in the array

    fruits.insert(2, "Anotherfruit");
    println!("The array after inserting the element at the specified position in the array is {:?}", fruits);


    // we can remove the element from the array something like this 

    fruits.remove(0);
    println!("The array after removing the element from the specified position is: {:?}", fruits);


    // finding the length of the vector array 

    let anotherarray = vec!["Apple", "Banana", "Cherry", "Kiwi"];
    println!("The length of the vector array is: {}", anotherarray.len());


    let cars = vec!["Ford", "Tata", "BMW", "Mazda"];
    for i in &cars{
        println!("The car is: {}", i);
    }

    println!("{:?}", cars);
}