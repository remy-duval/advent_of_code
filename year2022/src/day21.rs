use std::collections::HashMap;

use commons::error::Result;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 21: Monkey Math";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data)?;
    println!("1. The root monkey will yell {first}");
    let second = second_part(&data)?;
    println!("2. The value to yell is {second}");

    Ok(())
}

fn first_part(input: &RootOperation) -> Result<i64> {
    fn evaluate(operation: &Operation) -> Result<i64> {
        match operation {
            Operation::Now(value) => Ok(*value),
            Operation::Later(tokens) => {
                let mut stack = vec![];
                for token in tokens.iter() {
                    match *token {
                        Token::Constant(v) | Token::Input(v) => stack.push(v),
                        Token::Operator(operator) => match (stack.pop(), stack.pop()) {
                            (Some(b), Some(a)) => stack.push(operator.apply(a, b)),
                            _ => return Err(err!("not enough operands in {tokens:?}")),
                        },
                    };
                }
                match *stack {
                    [result] => Ok(result),
                    _ => Err(err!("more than 1 token at the end in {tokens:?}")),
                }
            }
        }
    }

    let a = evaluate(&input.left)?;
    let b = evaluate(&input.right)?;
    Ok(input.operator.apply(a, b))
}

fn second_part(input: &RootOperation) -> Result<i64> {
    match input {
        RootOperation {
            left: Operation::Now(value),
            right: Operation::Later(tokens),
            ..
        }
        | RootOperation {
            left: Operation::Later(tokens),
            right: Operation::Now(value),
            ..
        } => {
            let mut constant = *value;
            let mut expr = tokens.as_slice();
            loop {
                expr = match &expr {
                    [expr @ .., Token::Constant(a), Token::Operator(ope)] => {
                        match ope {
                            Operator::Add => constant -= a,
                            Operator::Subtract => constant += a,
                            Operator::Multiply => constant /= a,
                            Operator::Divide => constant *= a,
                        };
                        expr
                    }
                    [Token::Constant(a), expr @ .., Token::Operator(ope)] => {
                        match ope {
                            Operator::Add => constant -= a,
                            Operator::Subtract => constant = a - constant,
                            Operator::Multiply => constant /= a,
                            Operator::Divide => constant /= a,
                        };
                        expr
                    }
                    [Token::Input(_)] => break Ok(constant),
                    other => break Err(err!("the equation input was not solved ({other:?})")),
                };
            }
        }
        _ => Err(err!("one side of root should be a constant, got {input:?}")),
    }
}

#[derive(Debug)]
struct RootOperation {
    left: Operation,
    operator: Operator,
    right: Operation,
}

/// Represents a token in the reverse polish notation of an expression
/// 1 2 + becomes Constant(1), Constant(2), Operator(+)
#[derive(Debug, Clone)]
enum Token {
    Constant(i64),
    Input(i64),
    Operator(Operator),
}

#[allow(clippy::box_collection)]
#[derive(Debug, Clone)]
enum Operation {
    /// The actual computed value that does not require the input
    Now(i64),
    /// A list of tokens to evaluate the expression in the reverse polish notation
    /// Boxing makes this smaller on the stack, useful as we have a big `Vec<Option<Eval>>`
    Later(Box<Vec<Token>>),
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<RootOperation> {
    fn name_id(name: &str) -> Result<u32> {
        if name.len() > 4 {
            Err(err!("too many characters (expected 4)"))
        } else {
            let mut result = [0u8; 4];
            name.bytes()
                .zip(&mut result)
                .for_each(|(b, dest)| *dest = b);
            Ok(u32::from_ne_bytes(result))
        }
    }
    enum Raw {
        Constant(i64),
        Input(i64),
        Operation { a: u32, b: u32, operator: Operator },
    }

    s.lines()
        .map(|line| {
            line.split_once(':')
                .wrap_err_with(|| format!("missing ':' in {line}"))
                .and_then(|(name, operation)| {
                    let name = name.trim();
                    let id = name_id(name)?;
                    let operation = operation.trim();
                    if let Some((i, ope)) = operation.match_indices(['+', '-', '*', '/']).next() {
                        let a = name_id(operation[..i].trim())?;
                        let b = name_id(operation[(i + 1)..].trim())?;
                        let operator = match ope.chars().next() {
                            Some('+') => Operator::Add,
                            Some('-') => Operator::Subtract,
                            Some('*') => Operator::Multiply,
                            _ => Operator::Divide,
                        };
                        let operation = Raw::Operation { a, b, operator };
                        Ok((id, operation))
                    } else {
                        operation
                            .parse::<i64>()
                            .map(|value| match name {
                                "humn" => (id, Raw::Input(value)),
                                _ => (id, Raw::Constant(value)),
                            })
                            .wrap_err_with(|| format!("bad operation in {line}"))
                    }
                })
        })
        .collect::<Result<HashMap<u32, Raw>>>()
        .and_then(|operations| {
            let (a, b, operator) = match operations.get(&name_id("root")?) {
                Some(Raw::Operation { a, b, operator }) => (*a, *b, *operator),
                _ => return Err(err!("root is not an operation")),
            };

            let mut values: HashMap<u32, Operation> = HashMap::with_capacity(operations.len());
            let mut stack = vec![a, b];
            while let Some(name) = stack.pop() {
                let operation = match operations.get(&name) {
                    Some(Raw::Constant(v)) => Operation::Now(*v),
                    Some(Raw::Input(v)) => Operation::Later(Box::new(vec![Token::Input(*v)])),
                    Some(Raw::Operation { a, b, operator }) => {
                        match (values.get(a).cloned(), values.get(b)) {
                            (Some(a), Some(b)) => match a {
                                Operation::Now(a) => match b {
                                    Operation::Now(b) => Operation::Now(operator.apply(a, *b)),
                                    Operation::Later(b) => {
                                        let mut result = Vec::with_capacity(b.len() + 2);
                                        result.push(Token::Constant(a));
                                        result.extend_from_slice(b);
                                        result.push(Token::Operator(*operator));
                                        Operation::Later(Box::new(result))
                                    }
                                },
                                Operation::Later(mut result) => {
                                    match b {
                                        Operation::Now(b) => result.push(Token::Constant(*b)),
                                        Operation::Later(b) => result.extend_from_slice(b),
                                    };
                                    result.push(Token::Operator(*operator));
                                    Operation::Later(result)
                                }
                            },
                            (opt_a, opt_b) => {
                                stack.push(name); // Re-compute the value later
                                if opt_a.is_none() {
                                    stack.push(*a);
                                }
                                if opt_b.is_none() {
                                    stack.push(*b);
                                }
                                continue;
                            }
                        }
                    }
                    None => return Err(err!("unknown names in operation")),
                };
                values.insert(name, operation);
            }

            match (values.remove(&a), values.remove(&b)) {
                (Some(left), Some(right)) => Ok(RootOperation {
                    left,
                    operator,
                    right,
                }),
                _ => Err(err!("could not resolve both sides of root")),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/21.txt");
    const MAIN: &str = include_str!("../inputs/21.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 152);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 75_147_370_123_646);
    }

    #[test]
    fn second_part_example() {
        // 4 2 INPUT 3 - * + 4 / = 150
        // 4 2 INPUT 3 - * + = 150 4 *
        // 2 INPUT 3 - * = 150 4 * 4 -
        // INPUT 3 - = 150 4 * 4 - 2 /
        // INPUT = 150 4 * 4 - 2 / 3 +
        // INPUT = 150 4 * 4 - 2 / 3 +
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 301);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 3_423_279_932_937);
    }
}
