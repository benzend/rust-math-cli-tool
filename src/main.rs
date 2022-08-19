use clap::{Parser, Subcommand};

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

            // let multiplied = Vec::<MathsArg>::new();

            // let mut before: Option<MathsArg> = None;
            // let mut middle: Option<MathsArg> = None;
            // for current in refactored.into_iter() {
            //     match (&before, &middle) {
            //         (Some(b), Some(m)) => match (b, m, &current) {
            //             (MathsArg::Int(b), MathsArg::Op(Operator::Times), MathsArg::Int(m)) => {
            //                 MathsArg::Int(b * m);
            //             }
            //             _ => {}
            //         },
            //         _ => {}
            //     }

            //     before = middle;
            //     middle = Some(current);
            // }

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

            // println!("Result: {}", result);
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

    validated
}
