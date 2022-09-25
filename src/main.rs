use std::io;

fn main() {
    println!("\nThis program converts temperatures from Fahrenheit to Celsius or Celsius to Fahrenheit.\n");

    loop {
        println!("Enter a value to convert its temperature unit");
        let mut user_value = String::new();
        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line.");
        let user_value: f32 = user_value.trim().parse().expect("Please type a number!");

        println!("Type F for F to C or Type C for C to F");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line.");

        let user_choice = user_choice.trim();

        if user_choice == "F" {
            let converted_value: f32 = (user_value - 32.0) / 1.8;
            println!("You are converting {user_value} Fahrenheit to Celsius");
            println!("{user_value} Fahrenheit is equal to {converted_value} Celsius");
        } else if user_choice == "C" {
            let converted_value: f32 = 1.8 * user_value + 32.0;
            println!("You are converting {user_value} Celsius to Fahrenheit");
            println!("{user_value} Celsius is equal to {converted_value} Fahrenheit");
        } else {
            println!("You entered an invalid respond.");
        }

        println!("\n\nDo you want to convert another: Y for yes or N for no");
        let mut user_continue = String::new();
        io::stdin()
            .read_line(&mut user_continue)
            .expect("Failed to read line.");
        let user_continue = user_continue.trim();
        if user_continue == "N" {
            break println!("Thanks for using the app, Bye");
        }
    }
}
