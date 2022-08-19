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
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Times,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Maths { equation } => {
            let split = equation.split(" ").collect::<Vec<&str>>();

            let mut result: u32 = 0;
            let mut group: (Option<u32>, Option<Operator>, Option<u32>) = (None, None, None);

            for item in split.into_iter() {
                if group.0 == None {
                    match item.parse::<u32>() {
                        Ok(res) => group.0 = Some(res),
                        Err(err) => panic!("{:?}", err),
                    }
                } else if group.1 == None {
                    let operator = match item {
                        "+" => Some(Operator::Plus),
                        "-" => Some(Operator::Minus),
                        "*" | "x" => Some(Operator::Times),
                        _ => None,
                    };

                    if let Some(op) = operator {
                        group.1 = Some(op)
                    } else {
                        panic!("arguments invalid: no valid operator")
                    }
                } else if group.2 == None {
                    match item.parse::<u32>() {
                        Ok(res) => group.2 = Some(res),
                        Err(err) => panic!("{:?}", err),
                    }

                    result = match group {
                        (Some(a), Some(operator), Some(b)) => match operator {
                            Operator::Plus => a + b,
                            Operator::Minus => a - b,
                            Operator::Times => a * b,
                        },
                        _ => panic!("application failure: arg grouping wasn't done correctly"),
                    };

                    group = (Some(result), None, None)
                }
            }

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
    }
}
