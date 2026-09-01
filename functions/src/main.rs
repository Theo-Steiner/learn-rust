use std::io;

fn get_max_min(bits: u8, is_signed: bool) -> (i64, u64) {
    let base: u32 = 2;
    if !is_signed {
        (0, base.pow(bits.into()) - 1);
    }
    let exponent: u32 = u32::from(bits) - 1;
    (
        i64::from(base.pow(exponent)) * -1,
        u64::from(base.pow(exponent)),
    )
}

fn which_integer_type(min: i64, max: i64) -> String {
    let is_signed = min < 0;
    let available_bits: [u8; 4] = [8, 16, 32, 64];
    for bits in available_bits {
        let (possible_min, possible_max) = get_max_min(bits, is_signed);
        if min < possible_min
            || max
                > possible_max
                    .try_into()
                    .expect("provide something smaller than i64 for max")
        {
            continue;
        }
        let prefix = if is_signed { "i" } else { "u" };
        return format!("{}{}", prefix, bits);
    }
    panic!("out of bounds")
}

// u8 -> max: 2^8 - 1 = 255
//    -> min: 0
// but for temparature, you want negative numbers... so i8?
// i8 -> max: 2^(8-1) - 1 = 127
//    -> min: 2^(8-1) * -1 = -128
// let's write a function for figuring out appropriate number types
// also... how the fuck do you convert this again
//
fn convert_to_sane_temperature_unit(fahrenheit: i64) -> i64 {
    (fahrenheit - 32) * 5 / 9
}

fn get_number_input(prompt: &str) -> i64 {
    loop {
        let mut number = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut number).expect("invalid input");
        match number.trim().parse::<i64>() {
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
        if selection.trim() == "int" {
            let min = get_number_input(
                "What's the minimum value you expect handling? (enter 0 if you don't need negative numbers)",
            );
            let max = get_number_input("What's the maximum value you expect handling?");
            let t = which_integer_type(min, max);
            println!("you should use: {}", t)
        } else {
            let fahrenheit = get_number_input("How many degrees Fahrenheit?");
            let celsius = convert_to_sane_temperature_unit(fahrenheit);
            println!(
                "{} degrees freedom are {} degrees sane",
                fahrenheit, celsius
            )
        }
    }
}

fn main() {
    cli();
}
