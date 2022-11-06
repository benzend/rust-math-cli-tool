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
        first_arg: i32,
        /// second int
        second_arg: i32,
    },
    #[clap(arg_required_else_help = true)]
    Subtract {
        /// first int
        first_arg: i32,
        /// second int
        second_arg: i32,
    },
    #[clap(arg_required_else_help = true)]
    Multiply {
        /// first int
        first_arg: i32,
        /// second int
        second_arg: i32,
    },
    #[clap(arg_required_else_help = true)]
    Divide {
        /// first int
        first_arg: i32,
        /// second int
        second_arg: i32,
    },
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divisor,
}

impl From<&Operator> for Operator {
    fn from(op: &Operator) -> Self {
        match op {
            Operator::Times => Operator::Times,
            Operator::Divisor => Operator::Divisor,
            Operator::Plus => Operator::Plus,
            Operator::Minus => Operator::Minus,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MathsArg {
    Op(Operator),
    Int(i32),
}

#[derive(Debug)]
struct Chain {
    op: Operator,
    nums: Vec<i32>,
    prepend: Option<Operator>,
}

impl Chain {
    fn new(op: Operator, init: Option<Vec<i32>>, prepend: Option<Operator>) -> Chain {
        let nums = if let Some(init) = init {
            init
        } else {
            Vec::new()
        };

        Chain { op, nums, prepend }
    }
}

trait Push<T> {
    fn push(&mut self, x: T);
}

impl Push<&MathsArg> for Chain {
    fn push(&mut self, x: &MathsArg) {
        match x {
            MathsArg::Int(x) => self.nums.push(*x),
            _ => panic!("Not a valid item to push to Chain"),
        }
    }
}

impl Push<i32> for Chain {
    fn push(&mut self, x: i32) {
        self.nums.push(x);
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Maths { equation } => {
            println!("Result: {}", parse_maths_equation(equation));
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
            other => match other.parse::<i32>() {
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

fn parse_maths_equation(equation: String) -> i32 {
    let split = equation.split(" ").collect::<Vec<&str>>();

    let validated = validate_maths_vector(parse_maths_vector(split));

    let result = if validated.len() == 3 {
        match (&validated[0], &validated[1], &validated[2]) {
            (MathsArg::Int(a), MathsArg::Op(op), MathsArg::Int(b)) => match op {
                Operator::Times => a * b,
                Operator::Divisor => a / b,
                Operator::Minus => a - b,
                Operator::Plus => a + b,
            },
            _ => panic!("Not a valid string"),
        }
    } else if validated.len() == 5 {
        match (
            &validated[0],
            &validated[1],
            &validated[2],
            &validated[3],
            &validated[4],
        ) {
            (
                MathsArg::Int(a),
                MathsArg::Op(op_a),
                MathsArg::Int(b),
                MathsArg::Op(op_b),
                MathsArg::Int(c),
            ) => match (op_a, op_b) {
                (Operator::Times, Operator::Times) => a * b * c,
                (Operator::Times, Operator::Divisor) => a * b / c,
                (Operator::Times, Operator::Plus) => (a * b) + c,
                (Operator::Times, Operator::Minus) => (a * b) - c,
                (Operator::Divisor, Operator::Times) => a / b * c,
                (Operator::Divisor, Operator::Divisor) => a / b / c,
                (Operator::Divisor, Operator::Plus) => (a / b) + c,
                (Operator::Divisor, Operator::Minus) => (a / b) - c,
                (Operator::Plus, Operator::Times) => a + (b * c),
                (Operator::Plus, Operator::Divisor) => a + (b / c),
                (Operator::Plus, Operator::Plus) => a + b + c,
                (Operator::Plus, Operator::Minus) => a + b - c,
                (Operator::Minus, Operator::Times) => a - (b * c),
                (Operator::Minus, Operator::Divisor) => a - (b / c),
                (Operator::Minus, Operator::Plus) => a - b + c,
                (Operator::Minus, Operator::Minus) => a - b - c,
            },
            _ => panic!("Not a valid string!"),
        }
    } else {
        let mut chained: Vec<Chain> = Vec::new();
        let mut prev_op: Option<Operator> = None;
        let mut arg_1: Option<&MathsArg> = None;

        for (validated_index, arg) in validated.iter().enumerate() {
            if let Some(prev_arg) = arg_1 {
                match (prev_arg, &arg) {
                    (MathsArg::Int(prev), MathsArg::Op(current)) => {
                        let chained_len = &chained.len();
                        if chained_len > &0 {
                            match (&chained[chained_len - 1].op, current, &prev_op) {
                                // * Handle 4 * 3 * 2 * 1
                                (Operator::Times, Operator::Times, Some(Operator::Times)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 * 3 * 2 + 1
                                (Operator::Times, Operator::Plus, Some(Operator::Times)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 * 3 * 2 - 1
                                (Operator::Times, Operator::Minus, Some(Operator::Times)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 * 3 * 2 / 1
                                (Operator::Times, Operator::Divisor, Some(Operator::Times)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 + 3 * 2 * 1
                                (Operator::Plus, Operator::Times, Some(Operator::Plus)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                // * Handle 4 - 3 * 2 * 1
                                (Operator::Minus, Operator::Times, Some(Operator::Minus)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                // * Handle 4 / 3 * 2 * 1
                                (Operator::Divisor, Operator::Times, Some(Operator::Divisor)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                // * Handle 4 * 3 + 2 * 1
                                (Operator::Times, Operator::Times, Some(Operator::Plus)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        Some(Operator::Plus),
                                    ));
                                }
                                // * Handle 4 * 3 - 2 * 1
                                (Operator::Times, Operator::Times, Some(Operator::Minus)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        Some(Operator::Minus),
                                    ));
                                }
                                // * Handle 4 * 3 / 2 * 1
                                (Operator::Times, Operator::Times, Some(Operator::Divisor)) => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        Some(Operator::Divisor),
                                    ));
                                }
                                // * Handle 4 + 3 + 2 + 1
                                (Operator::Plus, Operator::Plus, Some(Operator::Plus)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 - 3 - 2 - 1
                                (Operator::Minus, Operator::Minus, Some(Operator::Minus)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                // * Handle 4 / 3 / 2 / 1
                                (Operator::Divisor, Operator::Divisor, Some(Operator::Divisor)) => {
                                    chained[chained_len - 1].push(*prev);
                                }
                                _ => {}
                            }
                        } else {
                            match current {
                                Operator::Times => {
                                    chained.push(Chain::new(
                                        Operator::Times,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                Operator::Divisor => {
                                    chained.push(Chain::new(
                                        Operator::Divisor,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                Operator::Plus => {
                                    chained.push(Chain::new(
                                        Operator::Plus,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                                Operator::Minus => {
                                    chained.push(Chain::new(
                                        Operator::Minus,
                                        Some(vec![*prev]),
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                    (MathsArg::Op(op), MathsArg::Int(x)) => {
                        if validated_index == &validated.len() - 1 {
                            let chained_len = &chained.len();

                            match (&chained[chained_len - 1].op, op) {
                                (Operator::Times, Operator::Times)
                                | (Operator::Plus, Operator::Plus)
                                | (Operator::Minus, Operator::Minus)
                                | (Operator::Divisor, Operator::Divisor) => {
                                    chained[chained_len - 1].push(*x);
                                }
                                _ => {
                                    chained.push(Chain::new(
                                        Operator::from(op),
                                        Some(vec![*x]),
                                        None,
                                    ));
                                }
                            }
                        }
                    }

                    _ => {}
                }
            };

            match &arg {
                MathsArg::Op(op) => {
                    prev_op = Some(Operator::from(op));
                }
                _ => {}
            }
            arg_1 = Some(&arg);
        }

        println!("chained {:?}", chained);
        1
    };

    result
}

mod tests {
    mod maths {

        #[test]
        fn handles_any_operator_with_two_args() {
            use crate::parse_maths_equation;

            struct EquationWithResult {
                equation: String,
                result: i32,
            }

            impl EquationWithResult {
                fn new(equation: String, result: i32) -> EquationWithResult {
                    EquationWithResult { equation, result }
                }
            }

            let simple_equations = vec![
                EquationWithResult::new("10 + 2".to_string(), 12),
                EquationWithResult::new("2 - 1".to_string(), 1),
                EquationWithResult::new("20 - 1".to_string(), 19),
                EquationWithResult::new("2 - 10".to_string(), -8),
                EquationWithResult::new("10 / 2".to_string(), 5),
                EquationWithResult::new("21 / 7".to_string(), 3),
                EquationWithResult::new("10 * 2".to_string(), 20),
                EquationWithResult::new("10 * -2".to_string(), -20),
            ];

            simple_equations.into_iter().for_each(|test| {
                assert_eq!(test.result, parse_maths_equation(test.equation));
            })
        }

        #[test]
        fn handles_order_of_ops() {
            use crate::parse_maths_equation;

            struct MathsExpectation {
                input: String,
                output: i32,
            }

            let tests = vec![
                MathsExpectation {
                    input: "3 * 3".to_string(),
                    output: 9,
                },
                MathsExpectation {
                    input: "1 + 5 * 2".to_string(),
                    output: 11,
                },
            ];

            tests.into_iter().for_each(|t| {
                assert_eq!(parse_maths_equation(t.input), t.output);
            })
        }
    }
}
