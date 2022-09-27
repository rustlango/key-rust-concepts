fn main() {
    println!("Hello, world!");
    basic_functions::functions_in_rust();
}

mod basic_functions { pub fn functions_in_rust() {
    println!("Hello, world!");

    second_function();

}

fn second_function() {
    println!("A second function");
}
}

