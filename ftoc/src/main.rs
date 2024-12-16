// FtoC: convert Fahrenheit to Celsius, or vice versa
use std::io;

fn main() {
    // Accepts user input and prints the resulting conversion
    let fahrenheit = convert_from_f();

    println!("What temperature should I convert?");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    
    if fahrenheit {
        println!("\n{}°F is equivalent to {}°C", input, f_to_c(input))
    }
    else {
    println!("\n{}°C is equivalent to {}°F", input, c_to_f(input))
    }
}

fn convert_from_f() -> bool {
    // Returns true if converting from Fahrenheit, otherwise False 
    loop {
        println!("Convert from Fahrenheit or Celsius?");
        
        let mut units = String::new();
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");
            
        let units = units.trim().to_lowercase();
        
        if units == "f" {
            println!("Converting from Fahrenheit.");
            return true
        } 
        
        else if units == "c" {
            println!("Converting from Celsius.");
            return false
        } 
        
        else {
            println!("\nPlease enter F or C.");
        }
    }
}

fn f_to_c(f: f32) -> f32   {
    // Converts Fahrenheit to Celsius
    return (f - 32.0) * 5.0/9.0
}

fn c_to_f(c: f32) -> f32   {
    // Converts Celsius to Fahrenheit
    return (c * 9.0/5.0) + 32.0
}