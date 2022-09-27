fn main() {
    println!("Hello, world!");
    basic_functions::functions_in_rust();
    flow_control_statements::flow_control_statements();
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