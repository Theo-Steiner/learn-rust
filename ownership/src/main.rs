fn takes_ownership(str: String) {
    println!("took ownership of string: {str}")
}
fn makes_copy(scalar: u32) {
    println!("made copy of: {scalar}")
}

fn main() {
    let x = String::from("hello");
    // "move" of ownership
    // (in other languages, this would be a "shallow copy" - but since rust also invalidates x in
    // the process it's a move)
    let y = x;
    // println!("{x}, {y}")
    // ^ borrow of moved value: `x`. value borrowed here after move
    // "move occurs because string doesn't have the copy trait"
    println!("{y}");

    let mut s = String::from("hello");
    println!("{s}, world!");
    // when shadowing a variable, the original variable goes out of scope & the original value is "dropped" immediately
    s = String::from("ahoy");

    println!("{s}, world!");

    let s1 = String::from("hello");
    // explicit copy of heap data implemented via the clone method
    let s2 = s1.clone();
    println!("{s1}, {s2}");

    // for values that live entirely on the stack (like scalar values), the "copy" trait can be implemented - and it it
    // is invoked on reassignment
    let n1 = 6;
    let n2 = n1;
    // ^ n2 is a stack copy of n1 because u32 implements the copy trait
    println!("{n1}, {n2}");
    // type with the "copy" trait cannot implement the "drop" trait and vice versa

    makes_copy(n1);
    println!("{n1}");
    // since passing a scalar value as an argument to a function merely copies the value, we can
    // still access n1 after passing it as an argument.

    takes_ownership(s1);
    // types that don't implement the "copy" trait, like String, however are "moved" by passing them
    // as a function argument, so the below code is invalid:
    // println!("{s1}")
    // ^ borrow of moved value: `s1`. value borrowed here after move
}
