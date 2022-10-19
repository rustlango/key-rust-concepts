// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    call_me(3);
}

// Annotated function param with type u8 (always do this even with retrun types)
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
