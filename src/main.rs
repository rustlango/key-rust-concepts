use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // Basic functions
    basic_functions::functions_in_rust();

    // flow control statements

    flow_control_statements::flow_control_statements();
    loops_in_rust::loop_loop::loop_loop();
    loops_in_rust::while_loop::while_loop();

    // Enums and match statements
    enum_and_match::value_in_cents(enum_and_match::Coin::Penny);
    enum_and_match::value_in_cents(enum_and_match::Coin::Nickel);
    enum_and_match::value_in_cents(enum_and_match::Coin::Dime);
    enum_and_match::value_in_cents(enum_and_match::Coin::Quarter);

    // Vectors and HashMaps
    println!("The v vector looks like this {:?}", vecs_and_hashmaps::vecs::vectors_in_rust());

    println!("The scores hashmap looks like this {:?}", vecs_and_hashmaps::hashmaps::hashmapping());
}

mod basic_functions { pub fn functions_in_rust() {
    println!("Hello, world!");

    second_function();
    
}

fn second_function() {
    println!("A second function");
}
}

// FLOW CONTROL STATEMENTS

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

// LOOPS IN RUST

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

// Enum and match statements

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

// Collections: Vectors and Hash Maps

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