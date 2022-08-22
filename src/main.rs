use clap::{Parser, Subcommand};
use std::cmp::Ordering;

/// A mathmatical cli
#[derive(Debug, Parser)]
#[clap(name = "maths")]
#[clap(about = "A mathmatical cli", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Do math as a string
    #[clap(arg_required_else_help = true)]
    Maths {
        /// A mathmatical equation string
        equation: String,
    },
    /// Add
    #[clap(arg_required_else_help = true)]
    Add {
        /// first int
        first_arg: u32,
        /// second int
        second_arg: u32,
    },
    #[clap(arg_required_else_help = true)]
    Subtract {
        /// first int
        first_arg: u32,
        /// second int
        second_arg: u32,
    },
    #[clap(arg_required_else_help = true)]
    Multiply {
        /// first int
        first_arg: u32,
        /// second int
        second_arg: u32,
    },
    #[clap(arg_required_else_help = true)]
    Divide {
        /// first int
        first_arg: u32,
        /// second int
        second_arg: u32,
    },
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divisor,
}

#[derive(Debug, PartialEq)]
enum MathsArg {
    Op(Operator),
    Int(u32),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Maths { equation } => {
            let split = equation.split(" ").collect::<Vec<&str>>();

            let validated = validate_maths_vector(parse_maths_vector(split));

            let mut result: u32;

            result = if validated.len() == 3 as usize {
                match (&validated[0], &validated[1], &validated[2]) {
                    (MathsArg::Int(a), MathsArg::Op(op), MathsArg::Int(b)) => match op {
                        Operator::Times => a * b,
                        Operator::Divisor => a / b,
                        Operator::Minus => a - b,
                        Operator::Plus => a + b,
                    },
                    _ => panic!("Not a valid string"),
                }
            } else {
                let multiplied = multipy_maths_vector(validated);
                print!("{:?}", multiplied);

                3
            };

            // let mut result: u32 = 0;
            // let mut group: (Option<u32>, Option<Operator>, Option<u32>) = (None, None, None);

            // for item in split.into_iter() {
            //     if group.0 == None {
            //         match item.parse::<u32>() {
            //             Ok(res) => group.0 = Some(res),
            //             Err(err) => panic!("{:?}", err),
            //         }
            //     } else if group.1 == None {
            //         let operator = match item {
            //             "+" => Some(Operator::Plus),
            //             "-" => Some(Operator::Minus),
            //             "*" | "x" => Some(Operator::Times),
            //             "/" => Some(Operator::Divisor),
            //             _ => None,
            //         };

            //         if let Some(op) = operator {
            //             group.1 = Some(op)
            //         } else {
            //             panic!("arguments invalid: no valid operator")
            //         }
            //     } else if group.2 == None {
            //         match item.parse::<u32>() {
            //             Ok(res) => group.2 = Some(res),
            //             Err(err) => panic!("{:?}", err),
            //         }

            //         result = match group {
            //             (Some(a), Some(operator), Some(b)) => match operator {
            //                 Operator::Plus => a + b,
            //                 Operator::Minus => a - b,
            //                 Operator::Times => a * b,
            //                 Operator::Divisor => a / b,
            //             },
            //             _ => panic!("application failure: arg grouping wasn't done correctly"),
            //         };

            //         group = (Some(result), None, None)
            //     }
            // }

            println!("Result: {}", result);
        }
        Commands::Add {
            first_arg,
            second_arg,
        } => {
            println!("{}", first_arg + second_arg);
        }
        Commands::Subtract {
            first_arg,
            second_arg,
        } => {
            println!("{}", first_arg - second_arg);
        }
        Commands::Multiply {
            first_arg,
            second_arg,
        } => {
            println!("{}", first_arg * second_arg);
        }
        Commands::Divide {
            first_arg,
            second_arg,
        } => {
            println!("{}", first_arg / second_arg);
        }
    }
}

fn parse_maths_vector(vector: Vec<&str>) -> Vec<MathsArg> {
    vector
        .into_iter()
        .map(|item| match item {
            "+" => MathsArg::Op(Operator::Plus),
            "-" => MathsArg::Op(Operator::Minus),
            "*" | "x" => MathsArg::Op(Operator::Times),
            "/" => MathsArg::Op(Operator::Divisor),
            other => match other.parse::<u32>() {
                Ok(res) => MathsArg::Int(res),
                Err(_) => panic!("couldn't parse value in arg list"),
            },
        })
        .collect::<Vec<MathsArg>>()
}

fn validate_maths_vector(vector: Vec<MathsArg>) -> Vec<MathsArg> {
    let mut validated = Vec::<MathsArg>::new();

    for (i, item) in vector.into_iter().enumerate() {
        if i % 2 == 0 {
            // * Operators should always be at an even index
            match item {
                MathsArg::Int(_) => {}
                MathsArg::Op(_) => panic!("Operators are out of order"),
            }
        } else {
            // * Integers should always be at an even index
            match item {
                MathsArg::Op(_) => {}
                MathsArg::Int(_) => panic!("Integers are out of order"),
            }
        }

        validated.push(item)
    }

    if validated.len() % 2 == 0 {
        panic!("Even number of arguments")
    }

    validated
}

fn multipy_maths_vector(vector: Vec<MathsArg>) -> Vec<MathsArg> {
    let mut new_vector = Vec::<MathsArg>::new();
    let mut first_arg = None;
    let mut second_arg = None;

    let validated_len = vector.len();

    let mut was_mulitplied = false;
    for (i, third_arg) in vector.into_iter().enumerate() {
        let new_arg = match (&first_arg, &second_arg, &third_arg) {
            (Some(MathsArg::Int(a)), Some(MathsArg::Op(op)), MathsArg::Int(b)) => {
                match op {
                    Operator::Times => {
                        if i == 2 {
                            // new_vector.push(MathsArg::Int(a * b));
                            was_mulitplied = true;
                            Some(MathsArg::Int(a * b))
                        } else if validated_len - 1 == i {
                            new_vector.push(MathsArg::Int(a * b));
                            None
                        } else {
                            was_mulitplied = true;
                            Some(MathsArg::Int(a * b))
                        }
                    }
                    Operator::Divisor => {
                        if i == 2 || was_mulitplied || validated_len - 1 == i {
                            new_vector.push(MathsArg::Int(*a));
                            new_vector.push(MathsArg::Op(Operator::Divisor));
                            new_vector.push(MathsArg::Int(*b));
                        } else if validated_len - 3 == i {
                            new_vector.push(MathsArg::Op(Operator::Divisor));
                        } else {
                            new_vector.push(MathsArg::Op(Operator::Divisor));
                            new_vector.push(MathsArg::Int(*b));
                        }
                        was_mulitplied = false;
                        None
                    }
                    Operator::Plus => {
                        if i == 2 || was_mulitplied || validated_len - 1 == i {
                            new_vector.push(MathsArg::Int(*a));
                            new_vector.push(MathsArg::Op(Operator::Plus));
                            new_vector.push(MathsArg::Int(*b));
                        } else if validated_len - 3 == i {
                            new_vector.push(MathsArg::Op(Operator::Plus));
                        } else {
                            new_vector.push(MathsArg::Op(Operator::Plus));
                            new_vector.push(MathsArg::Int(*b));
                        }
                        was_mulitplied = false;
                        None
                    }
                    Operator::Minus => {
                        if i == 2 || was_mulitplied || validated_len - 1 == i {
                            new_vector.push(MathsArg::Int(*a));
                            new_vector.push(MathsArg::Op(Operator::Minus));
                            new_vector.push(MathsArg::Int(*b));
                        } else if validated_len - 3 == i {
                            new_vector.push(MathsArg::Op(Operator::Minus));
                        } else {
                            new_vector.push(MathsArg::Op(Operator::Minus));
                            new_vector.push(MathsArg::Int(*b));
                        }
                        was_mulitplied = false;
                        None
                    }
                }
            }
            _ => None,
        };

        first_arg = second_arg;

        if let Some(new_arg) = new_arg {
            second_arg = Some(new_arg)
        } else {
            second_arg = Some(third_arg)
        }
    }

    new_vector
}
