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

fn first_part(input: &Input) -> Result<i64> {
    let (a, b, operator) = input.compute_root_node(
        std::convert::identity,
        std::convert::identity,
        |a, b, operator| operator.apply(a, b),
    )?;

    Ok(operator.apply(a, b))
}

/// Represents a token in the reverse polish notation of an expression
/// 1 2 + becomes Constant(1), Constant(2), Operator(+)
#[derive(Debug, Clone)]
enum Token {
    Constant(i64),
    Input,
    Operator(Operator),
}

#[allow(clippy::box_collection)]
#[derive(Debug, Clone)]
enum Eval {
    /// The actual computed value that does not require the input
    Now(i64),
    /// A list of tokens to evaluate the expression in the reverse polish notation
    /// Boxing makes this smaller on the stack, useful as we have a big `Vec<Option<Eval>>`
    Later(Box<Vec<Token>>),
}

fn second_part(input: &Input) -> Result<i64> {
    let (a, b, _) = input.compute_root_node(
        Eval::Now,
        |_| Eval::Later(Box::new(vec![Token::Input])),
        |a, b, operator| match a {
            Eval::Now(a) => match b {
                Eval::Now(b) => Eval::Now(operator.apply(a, b)),
                Eval::Later(mut b) => {
                    b.insert(0, Token::Constant(a));
                    b.push(Token::Operator(operator));
                    Eval::Later(b)
                }
            },
            Eval::Later(mut a) => {
                match b {
                    Eval::Now(b) => a.push(Token::Constant(b)),
                    Eval::Later(mut b) => a.append(&mut b),
                };
                a.push(Token::Operator(operator));
                Eval::Later(a)
            }
        },
    )?;

    fn resolve(mut expr: &[Token], mut constant: i64) -> Result<i64> {
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
                [Token::Input] => break Ok(constant),
                other => break Err(err!("the expression input was not resolved ({other:?})")),
            };
        }
    }

    match (a, b) {
        (Eval::Now(a), Eval::Later(b)) => resolve(&b, a),
        (Eval::Later(a), Eval::Now(b)) => resolve(&a, b),
        (a, b) => Err(err!(
            "one side of root should be a constant expression, but got {a:?} and {b:?}"
        )),
    }
}

#[derive(Debug)]
struct Input {
    root_index: usize,
    operations: Vec<Operation<usize>>,
}

impl Input {
    /// Assemble the value of the two sides of the root node from the input
    /// Returns the resolved `(Left, Right, Operator)`
    fn compute_root_node<Value: Clone>(
        &self,
        mut on_constant: impl FnMut(i64) -> Value,
        mut on_input: impl FnMut(i64) -> Value,
        mut on_operation: impl FnMut(Value, Value, Operator) -> Value,
    ) -> Result<(Value, Value, Operator)> {
        let (a, b, operator) = match self.operations[self.root_index] {
            Operation::Operation { a, b, operator } => (a, b, operator),
            _ => return Err(err!("root is not an operation")),
        };

        let mut values: Vec<Option<Value>> = vec![None; self.operations.len()];
        let mut stack = vec![a, b];
        while let Some(index) = stack.pop() {
            values[index] = match self.operations[index] {
                Operation::Constant(v) => Some(on_constant(v)),
                Operation::Input(v) => Some(on_input(v)),
                Operation::Operation { a, b, operator } => match (&values[a], &values[b]) {
                    (Some(a), Some(b)) => Some(on_operation(a.clone(), b.clone(), operator)),
                    (opt_a, opt_b) => {
                        stack.push(index); // Re-compute the value later
                        if opt_a.is_none() {
                            stack.push(a);
                        }
                        if opt_b.is_none() {
                            stack.push(b);
                        }
                        continue;
                    }
                },
            }
        }

        match (values[a].take(), values[b].take()) {
            (Some(a), Some(b)) => Ok((a, b, operator)),
            _ => Err(err!("could not resolve both sides of root")),
        }
    }
}

#[derive(Debug)]
enum Operation<Name> {
    Constant(i64),
    Input(i64),
    Operation {
        a: Name,
        b: Name,
        operator: Operator,
    },
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

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Input> {
    let mut names: HashMap<&str, usize> = HashMap::new();
    let mut opes: Vec<Operation<&str>> = Vec::new();
    let mut root_index: Option<usize> = None;
    for line in s.lines() {
        if let Some((name, operation)) = line.split_once(':') {
            let name = name.trim();
            let operation = operation.trim();

            if name == "root" {
                root_index = Some(names.len());
            }
            names.insert(name, names.len());
            if let Some((i, ope)) = operation.match_indices(['+', '-', '*', '/']).next() {
                let a = operation[..i].trim();
                let b = operation[(i + 1)..].trim();
                let operator = match ope.chars().next() {
                    Some('+') => Operator::Add,
                    Some('-') => Operator::Subtract,
                    Some('*') => Operator::Multiply,
                    _ => Operator::Divide,
                };
                opes.push(Operation::Operation { a, b, operator });
            } else if let Ok(value) = operation.parse::<i64>() {
                opes.push(if name == "humn" {
                    Operation::Input(value)
                } else {
                    Operation::Constant(value)
                });
            } else {
                return Err(err!("bad operation in {line}"));
            }
        } else {
            return Err(err!("missing ':' in {line}"));
        }
    }

    // Resolve names in operations to indexes in the operations vec
    let root_index = root_index.wrap_err("missing 'root' operation'")?;
    let mut operations: Vec<Operation<usize>> = Vec::with_capacity(opes.len());
    for ope in opes {
        let operation = match ope {
            Operation::Constant(value) => Operation::Constant(value),
            Operation::Input(value) => Operation::Input(value),
            Operation::Operation { a, b, operator } => {
                if let (Some(&a), Some(&b)) = (names.get(a), names.get(b)) {
                    Operation::Operation { a, b, operator }
                } else {
                    return Err(err!("unknown names {a} and {b} in operation"));
                }
            }
        };
        operations.push(operation);
    }

    Ok(Input {
        root_index,
        operations,
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
