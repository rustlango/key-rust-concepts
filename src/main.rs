use std::{collections::HashMap};


fn main() {
    println!("Hello, world!");

    // 1. Basic functions
    basic_functions::functions_in_rust();

    // 2. Flow control statements

    flow_control_statements::flow_control_statements();

    // 3. Loops in Rust

    loops_in_rust::loop_loop::loop_loop();
    loops_in_rust::while_loop::while_loop();

    // 4. Enums and match statements
    enum_and_match::value_in_cents(enum_and_match::Coin::Penny);
    enum_and_match::value_in_cents(enum_and_match::Coin::Nickel);
    enum_and_match::value_in_cents(enum_and_match::Coin::Dime);
    enum_and_match::value_in_cents(enum_and_match::Coin::Quarter);

    // 5. Collections: Vectors and HashMaps
    println!("The v vector looks like this {:?}", vecs_and_hashmaps::vecs::vectors_in_rust());

    println!("The scores hashmap looks like this {:?}", vecs_and_hashmaps::hashmaps::hashmapping());

    // 6. Macro rules

     hi_rustacean!();

     // 7. Generics: Cartesian coordinate system example

     let integer_coordinates  = Cartesianplane {x: 7, y: 28};
     print!("integer coordinates x: {} and y: {}\n", integer_coordinates.x, integer_coordinates.y);
     let float_cooridnates= Cartesianplane {x: 21, y: 42};
     print!("float coordinates x: {} and y: {}\n", float_cooridnates.x, float_cooridnates.y);
     let negative_float_coordinates = Cartesianplane {x: -14.07, y: -21.084};
     print!("negative float coordinates x: {} and y: {}\n", negative_float_coordinates.x, negative_float_coordinates.y);
     let integer_and_float = Cartesianplane {x: 35, y: 42.77};
     print!("integer and float coordinates x: {} and y: {}\n", integer_and_float.x, integer_and_float.y);

    // 8. TRAITS IN ACTION

    let ksm = Ksm {};
    let dot = Dot {};

    ksm.crypto_payment();
    dot.crypto_payment();

}

// Crypto structs for Kuasama and Polkadot crypto wallets
struct Ksm {}
struct Dot {}


// Crypto payment processor trait
trait Paycrypto {
    fn crypto_payment(&self);
}

// Implementation of Paycrypto trait extended onto the two crypto wallets
impl Paycrypto for Ksm {

    fn crypto_payment(&self) {
        println!("KSM wallet payment processed");
    }
}

impl Paycrypto for Dot {

    fn crypto_payment(&self) {
        println!("DOT wallet payment processed");
    }

}

// 1. FUNCTIONS IN RUST

mod basic_functions { pub fn functions_in_rust() {
    println!("Hello, world!");

    second_function();
    
}

fn second_function() {
    println!("A second function");
}
}

// 2. FLOW CONTROL STATEMENTS

mod flow_control_statements {
    pub fn flow_control_statements() {
        let number = 65;

        if number % 7 == 0 {
            println! ("number is divisible by 7");
        } else if number % 5 ==0 {
            println!("number is divisible by 5");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!{"number is not divisible by 7, 5 and 2"};
        }
    }
}

// 3. LOOPS IN RUST

mod loops_in_rust {
    pub mod loop_loop { 
        pub fn loop_loop() {
            let mut i:i32 = 1;
            
            let result = loop {
                if i==10 {
                   break i;
               }
               i +=1;
        };

        println!("The value of i is now {}", result);
         }

    }

    pub mod while_loop {
        pub fn while_loop() {
            let mut t: i32 = 1;
            while t<=10 {
                println!("The value of t is {}", t);
                t +=1;
            }
        }
    }
}

// 4. Enum and match statements

mod enum_and_match {

    // create custom datat type which is an enum of different coins
    pub enum Coin {
        Penny,
        Nickel, 
        Dime,
        Quarter
    }

    pub fn value_in_cents(coin: Coin) {
        match coin {
            Coin::Penny => println!("Penny in cents is: {}",1),
            Coin::Nickel => println!("Nickel in cents is: {}",5),
            Coin::Dime => println!("Dime in cents is: {}",10),
            Coin::Quarter => println!("Quarter in cents is: {}",25),
        }
    }
}

// 5. Collections: Vectors and Hash Maps

mod vecs_and_hashmaps {
    use super::*;
    pub mod vecs {

        pub fn vectors_in_rust() -> Vec<i32> {

        // vectors are growable arrays
        // always ensure you use let statements wrapped in functions
        let mut v: Vec<i32> = Vec::new();
            v.push(5);
            v.push(6);
            v.push(7);
            v.push(8);

            return v;
        }
    }

    pub mod hashmaps {
        use super::*;

        pub fn hashmapping() -> HashMap<String, f64> {
        
        // always ensure you use let statements wrapped in functions
        // basic hashmap implmentation for Polkadot and Kusama token balances.
        // So you can indeed use hashamps for crypto_wallet implmentations
    
        let mut crypto_balances = HashMap::new();

        crypto_balances.insert(String::from("DOT"), 105665.546655797706533222);
        crypto_balances.insert(String::from("KSM"), 4668.857054878686707642);

        return crypto_balances;
    }
 }
}

// 6. Macros
// A computing paradigm called metaprogramming - which basically means code 
// that writes other code. Macros are there to reduce redundancy and 
// boilerplate in code so that you do not have to write all the time
// You could have a rust greeter app.


mod basic_macro {
    #[macro_export] macro_rules! hi_rustacean {
    // `It takes zero arguments.
    () => {
        // The macro manifests into the the code below
        println!("Hello, Rustacean!");
    };
}
}

// 7. GENERICS
// Generics offer a way to create generalisable type definitions for enums,
//  traits, functions, maps and structs

    struct Cartesianplane<T, U> {
    x: T,
    y: U,
}

// 8. TRAITS:
// Traits affordrust developers to loosely or tightly enforuce shared 
// functionality on types such as structs

