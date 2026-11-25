fn main(){
    println!("Hello world");

    let mut count = 1;

    while count <= 5{
        println!("Count is :{} ", count);
        count = count + 1;
    }

    let mut num = 1;

    while num < 45 {
        if num == 23 {
            break;
        }
        println!("The loop is broke at the number: {} ", num);
        num = num + 1;
    }

    // skipping a value

    let mut anothernum = 1;

    while anothernum < 45 {
        if anothernum == 23{
            anothernum = anothernum + 1;
            continue ;
        }
        
        println!("The continues value is: {} ", anothernum);
        anothernum = anothernum + 1;
    }
}