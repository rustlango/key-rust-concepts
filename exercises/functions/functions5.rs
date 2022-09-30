// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// DONE WITH EXERCISE

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // removed semicolon o implicit return expression
    num * num
}
