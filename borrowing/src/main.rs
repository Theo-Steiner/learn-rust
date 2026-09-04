// since calling this function moves the string argument, we have to move it back by returning it,
// resulting in us having to return a tuple
fn annoying_calculate_length(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}

// prefixing a type with ampersand gives us a reference of that type
// a reference is the same thing as a "pointer" but with additional guarantees that the value of the
// type is actually where the pointer points too
fn calculate_length(s: &String) -> usize {
    s.len()
} // the original string is not dropped here because s just holds a reference & doesn't have
// ownership of s1

fn change(s: &mut String) {
    s.push_str("hello, ");
    // ^ if it were just & and not &mut -> cannot borrow `*s` as mutable, as it is behind a `&` reference
}

fn main() {
    let s1 = String::from("Hello, world!");
    let (length1, s1) = annoying_calculate_length(s1);
    println!("{s1} is {length1} chars long");
    // let reference = &s1;
    let length2 = calculate_length(&s1);
    println!("calculating the length via a reference gives us the same result... duh: {length2}");
    let mut s2 = String::from("Hello, ");
    let r1 = &mut s2;
    // let r2 = &mut s2;
    // ^ cannot borrow `s2` as mutable more than once at a time
    // rust blocks us having two mutable references at the same time to prevent data races
    change(r1);
}
