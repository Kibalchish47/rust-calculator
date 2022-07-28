use std::io::{self, Write};

enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}

impl Operation {
    fn evaluate(&self, num1: u32, num2 : u32) -> u32 { // this function defines what is the operation chosen by the user
        match self {
            Operation::Add  => num1 + num2, // returns num1 + num2 
            Operation::Sub  => num1 - num2, // returns ... -
            Operation::Mult => num1 * num2, // returns ... *
            Operation::Div  => num1 / num2, // returns ... /
        }
    }

    fn from_string(operation: String) -> Result<Self, String> {
        match operation.trim() {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mult),
            "/" => Ok(Operation::Div),
            &_ => Err("Error. ".to_string()), // error handling
        }
    }
}

struct Input;

impl Input 
{
    pub fn string(msg : &str) -> String {
        let mut input:String = String::new();
        print!("{}", msg);
    
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        return input;
    }
    
    fn u32(msg: &str) -> u32 { // this function checks if the input is a number to begin with
        Self::string(msg).trim().parse().expect("Not a number!")
    }
}

pub fn main() 
{
    println!("TEXT-BASED CALCULATOR");
    loop{
        println!("");
        let num1 = Input::u32("Enter the first number : ");
        let num2 = Input::u32("Enter the second number : ");

        let operation = Input::string("Choose your operation (+ - * /) : ");

        let operation = Operation::from_string(operation.to_string()).unwrap();

        let result:u32 =  operation.evaluate(num1, num2);

        println!("");
        println!("The result of the calculation is : {}", result);
    }
}