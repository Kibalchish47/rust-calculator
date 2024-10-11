use std::{
    io::Write,
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

pub type Number = f64;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiation,
    Modulo,
}

impl Operation {
    fn op(&self) -> fn(Number, Number) -> Number {
        match self {
            Operation::Add => Number::add,
            Operation::Subtract => Number::sub,
            Operation::Multiply => Number::mul,
            Operation::Divide => Number::div,
            Operation::Exponentiation => Number::powf,
            Operation::Modulo => Number::rem,
        }
    }
}

pub struct OperationParseError;

// Type: Sets of Values
// Functions: Maps of types: A -> B

// Traits: Sets of Types
// Traits (Associated Types):       Map: Type -> Type
// Traits (Associated Constants):   Map: Type -> Value

// Pointers: Map: value -> value

// FromStr: {u8, u16, f64, f32, ... }
// FromStr::Err {u8, u16, f64, f32 } -> { U8Error, U16Error, F64Error, }

// FnOnce (trait) - functions that you can only call once.
// FnMut (trait) - functions that
// Fn (trait) - functions that can inspect their environment.
// fn (trait, but also not) - pure functions.

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            "**" => Operation::Exponentiation,
            "%" => Operation::Modulo,
            _ => return Err(OperationParseError),
        })
    }
}

// Values
// 'static -- maximal lifetime \exists 'static \in LTs (\forall 'lt \in LTs 'static >= 'lt)

fn parse<F, T>(msg: &str, validator: F) -> Result<T, Box<dyn std::error::Error>>
where
    T: FromStr,
    F: Fn(&str) -> Result<T, T::Err>,
{
    let mut input = String::new();

    const A: usize = 1;

    loop {
        print!("{msg}");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        if let Ok(op) = validator(input.trim()) {
            return Ok(op);
        }

        input.clear();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: Number = 0.0;

    loop {
        let func = |s: &str| {
            if s.to_uppercase() == "MEM" {
                Ok(memory)
            } else {
                Number::from_str(s)
            }
        };

        let op: Operation = parse("Enter the operation (+ - * / ** %): ", Operation::from_str)?;
        let a: f64 = parse("Enter the first number: ", func)?;
        let b: f64 = parse("Enter the second number: ", func)?;

        memory = op.op()(a, b);
        println!("Result: {:.5}", memory);
    }
}
