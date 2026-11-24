fn anotherfunc(num: i32)-> i32{
    return num * 45;
}

fn main(){
    let value = 45;
    let add = 5 + 3;
    let subtract = 5 - 3;
    let division = 5 / 2;
    let multiplication = 5 * 2;
    let remainder = 5 % 2;
    println!("The Square of {} is {}: ", value, anotherfunc(value));
    println!("The addition is {} and subtraction is {} and division is {} and multiplication is {} and remainder is {}: ", add, subtract, division, multiplication, remainder);

    // Assignment operators

    let mut x = 3;
    println!("Start: {} ", x);

    x += 1;
    println!("After += 1: {} ", x);

    x -= 1;
    println!("After -= 1: {} ", x);

    x *= 1;
    println!("After *= 1: {} ", x);

    x /= 1;
    println!("After /= 1: {} ", x);
     
    x %= 2;
    println!("After %= 2: {} ", x);
}