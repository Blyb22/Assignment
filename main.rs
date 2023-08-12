use std::io;

fn main() {
    println!("Enter a number to print the table:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("fail to read number");
    let number: i32 = input.trim().parse().expect("input not an integer");

    for i in 1..11{
        let result = number*i;
        println!("{} x {} = {}",number,i,result)
        
    };
    

}
