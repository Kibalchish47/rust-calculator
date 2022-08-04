use std::io::{self, Write};
enum Operation { // every member is an operation : + - * / 
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    Mod, 
}

impl Operation {
    fn evaluate(&self, num1: u32, num2 : u32) -> u32 { // this function defines what is the operation chosen by the user
        match self {
            Operation::Add  => num1 + num2,    // returns num1 + num2 
            Operation::Sub  => num1 - num2,    // returns ... -
            Operation::Mult => num1 * num2,    // returns ... *
            Operation::Div  => num1 / num2,    // returns ... /
            Operation::Exp  => num1.pow(num2), // returns ... **
            Operation::Mod  => num1 % num2,    // returns ... %
        }
    }

    fn from_string(operation: String) -> Result<Self, String> { // associates the inputted operations with the operations in the enum
        match operation.trim() {
            "+"  => Ok(Operation::Add),  // does an addition 
            "-"  => Ok(Operation::Sub),  // does a substraction
            "*"  => Ok(Operation::Mult), // does a multiplication 
            "/"  => Ok(Operation::Div),  // does a division
            "**" => Ok(Operation::Exp),  // does an exponent 
            "%"  => Ok(Operation::Mod),  // does a modulus
            &_   => Err("Error. ".to_string()), // error handling
        }
    }
}

struct Input;

impl Input // grouping together helper functions 
{
    pub fn string(msg : &str) -> String { // takes an input (num1 and num2) and returns it 
        let mut input : String = String::new();
        print!("{}", msg);
    
        io::stdout().flush().unwrap(); // input 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        return input; // return input
    }
    
    fn u32(msg: &str) -> Option<u32> { // this function checks if the input is a number to begin with
        match Self::string(msg).trim() {
            // Load value from memory 
            "m" => None,
            // If it's not "m", normal parsing 
             s  => Some(s.parse().expect("Not a number!")),
        }
    }
}

pub fn main() 
{
    println!("TEXT-BASED CALCULATOR ! ");
    println!("To use the result of the previous calculation as a value, enter 'm' instead of the number (the value is 0 by default) !");

    let mut memory:u32 = 0;
    loop
    { // the actual input loop 
        println!("");
        
        // num 1 and num 2 input 
        let num1 = Input::u32("Enter the first number : ").unwrap_or(memory);
        let num2 = Input::u32("Enter the second number : ").unwrap_or(memory);

        //operation input
        let operation = Input::string("Choose your operation (+ - * / ** %) : ");

        let operation = Operation::from_string(operation.to_string()).unwrap();

        let result:u32 =  operation.evaluate(num1, num2); // this function actually does the maths 
        
        // result output
        println!("");
        println!("The result of the calculation is : {} (it is a u32)!", result);

        // setting the memory for the next iteration of the loop
        memory = result; 
    }
}