use std::io;

fn which_integer_type(min: i64, max: i64) -> String {
    String::from("u64")
}

// u8 -> max: 2^8 - 1 = 255
//    -> min: 0
// but for temparature, you want negative numbers... so i8?
// i8 -> max: 2^(8-1) - 1 = 127
//    -> min: 2^(8-1) * -1 = -128
// let's write a function for figuring out appropriate number types
fn convert_to_sane_temperature_unit(fahrenheit: u8) -> u8 {
    fahrenheit
}

fn get_number_input(prompt: &str) -> i64 {
    loop {
        let mut number = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut number).expect("invalid input");
        match number.parse::<i64>() {
            Ok(n) => return n,
            Err(_) => println!("input a valid number"),
        }
    }
}

fn cli() {
    loop {
        println!(
            "Which tool do you want to use?\ntype 'int' for a tool that helps you figure out the appropriate int type for your requirements,\n or 'tmp' for a converter of temperature to fahrenheit.\ntype anything else to exit."
        );
        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Err(_) => println!("Invalid input"),
            Ok(_) => println!("Okay!"),
        }
        if selection == "int" {
            let min = get_number_input(
                "What's the minimum value you expect handling? (enter 0 if you don't need negative numbers)",
            );
            let max = get_number_input("What's the maximum value you expect handling?");
            which_integer_type(min, max);
        } else {
            let fahrenheit = get_number_input("How many degrees Fahrenheit?");
            convert_to_sane_temperature_unit(fahrenheit);
        }
    }
}

fn main() {
    cli();
}
