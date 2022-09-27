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