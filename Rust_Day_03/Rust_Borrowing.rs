fn main(){
    println!("Hello world");

    let a = String::from("Ayush");
    let b = &a;

    println!("{}", a); 
    println!("{}", b); 


    let mut name = String::from("Ayush");
    let ref_name = &mut name;
    ref_name.push_str(" Done");
    println!("{}", ref_name);
}