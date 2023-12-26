fn main() {
    let mut x = 5;
    println!("The valuee of x is: {}", x);
    x = 10;
    println!("Now the value of x is: {}", x);

    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);
    
    println!("========================");
    let number: u32 = 42; // unsigned 32-bit integer
    let character: char = 'A'; // character 

    let float_num = 3.14; // Type inference - Rust can infer a type
    let boolean = true; 

    println!("Number: {}", number);
    println!("Character: {}", character);
    println!("Float num: {}", float_num);
    println!("Bool: {}", boolean);

    let mut num = 200;
    if num % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
    
    
    
    
}
