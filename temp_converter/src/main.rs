use std::io;

fn main() {
    println!("Welcome to temp conversion");

    loop {

        println!("What do you want the temp to be converted to?");
        println!("Enter C for Celsius or F for Fahrenheit");
    
        let mut output_format: String = String::new();
    
        io::stdin()
            .read_line(&mut output_format)
            .expect("Failed to read line");

        output_format = output_format.trim().to_lowercase();
    
        if output_format != "c" && output_format != "f" {
            println!("Please provide a valid input");
            continue;
        }
    
        println!("Your output format will be in {}", if output_format == "c" { "Celsius" } else { "Fahrenheit" });
    
        println!("Provide your input temp in {}", if output_format == "c" { "Fahrenheit" } else { "Celsius" });

        let mut input_temp: String = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read input temp");

        let input_temp: f64 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number");
                continue;
            }
        };

        println!("Your input temp {}{}", input_temp, if output_format == "c" { "F" } else { "C" });

        let output_temp: f64 = {
            if output_format == "c" {
                (input_temp - 32.0) * (5.0 / 9.0)
            }
            else {
                input_temp * (9.0 / 5.0) + 32.0
            }
        };

        println!("____________________");
        println!("The conversion result is: {:.2}{}", output_temp, output_format.to_uppercase());
        println!("____________________");
        println!("\n\n");
    }
}
