use std::io;

fn main() {
    println!("This program converts temperatures from F to C or C to F");

    loop {
        println!("Type F for F to C or Type C for C to F");

        let mut f_or_c = String::new();
        io::stdin().read_line(&mut f_or_c).expect("You entered an invalid answer");

        if f_or_c.trim() == "F" {
            println!("Converting Fahrenheit to Celsius");
            println!("Enter Fahrenheit value: ");       
        } else if f_or_c.trim() == "C" {
            println!("Converting Celsius to Fahrenheit");
            println!("Enter Celsius value: ");
        } else {
            println!("Invalid entry, try again {f_or_c}");
        }
    }
}
