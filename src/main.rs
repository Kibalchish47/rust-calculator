use std::io::{self, Write};

enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}

impl Operation {
    fn evaluate(&self, num1: u32, num2 : u32) -> u32 {
        match self {
            Operation::Add  => num1 + num2, // returns num1 + num2 
            Operation::Sub  => num1 - num2, // returns num1 - num2
            Operation::Mult => num1 * num2, // returns num1 * num2
            Operation::Div  => num1 / num2, // returns num1 / num2
        }
    }

    fn from_string(operation: String) -> Result<Self, String> {
        match operation.trim() {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mult),
            "/" => Ok(Operation::Div),
            &_ => Err("Operation Error. ".to_string()),
        }
    }
}

struct Input;

impl Input {
    pub fn string(msg : &str) -> String {
        let mut input:String = String::new();
        print!("{}", msg);
    
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Err: Failed to read line.");
    
        return input;
    }
    
    fn u32(msg: &str) -> u32 {
        Self::string(msg).trim().parse().expect("Not a number!")
    }
}

pub fn main() {
    println!("TEXT-BASED CALCULATOR on RUST");
    loop{
        println!("");
        let num1 = Input::u32("Enter the first number:~> ");
        let num2 = Input::u32("Enter the second number:~> ");

        let operation = Input::string("Choose your operation [+ - * /]:~> ");

        let operation = Operation::from_string(operation.to_string()).unwrap();

        let result : u32 =  operation.evaluate(num1, num2);

        println!("");
        println!("The result of the calculation is :~> {} .", result);
    }
}
