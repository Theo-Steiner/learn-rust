// actually computed at compile time => u32 10800
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = THREE_HOURS_IN_SECONDS;
    println!("The value of x is: {x}");
    let x = x + 6;
    let spaces = "   ";
    let spaces = spaces.len();
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is: {x}, and there are {spaces} spaces");
}
