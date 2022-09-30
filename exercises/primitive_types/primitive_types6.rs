// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand for a hint.

// DONE WITH EXERCISE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // use dot notation to access tuple elements - instead of square notation
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
