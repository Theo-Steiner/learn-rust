fn main() {
    let number = "3"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number: u8 = match number.trim().parse() {
        Err(_) => {
            println!("{number} is not a number");
            return;
        }
        Ok(n) => n,
    };
    println!("Number plus two is: {}", number + 2);
}
