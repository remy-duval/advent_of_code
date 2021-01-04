use std::str::FromStr;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Operation>;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 18: Operation Order";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let tokens = data.data;
        let first = first_part(&tokens)?;
        println!("No precedence: The sum of each line is {}", first);

        let second = second_part(&tokens)?;
        println!("Addition precedence: The sum of each line is {}", second);

        Ok(())
    }
}

fn first_part(tokens: &[Operation]) -> anyhow::Result<u64> {
    tokens.iter().map(Operation::evaluate_no_precedence).sum()
}

fn second_part(tokens: &[Operation]) -> anyhow::Result<u64> {
    tokens
        .iter()
        .map(Operation::evaluate_addition_has_precedence)
        .sum()
}

/// An operation to evaluate
#[derive(Debug, Clone)]
pub struct Operation(Vec<Token>);

impl Operation {
    /// Evaluate the operation with no precedence difference between + and *
    pub fn evaluate_no_precedence(&self) -> anyhow::Result<u64> {
        self.shunting_yard(|op| match op {
            Operator::Plus => 2,
            Operator::Mul => 2,
            Operator::OpenParen => 0,
            Operator::ClosingParen => 0,
        })
    }

    /// Evaluate the operation with a higher precedence for + than for *
    pub fn evaluate_addition_has_precedence(&self) -> anyhow::Result<u64> {
        self.shunting_yard(|op| match op {
            Operator::Plus => 3,
            Operator::Mul => 2,
            Operator::OpenParen => 0,
            Operator::ClosingParen => 0,
        })
    }

    /// A simplified version of the [Shunting Yard algorithm]
    ///
    /// ### Arguments
    /// * `precedence` - A closure to evaluate the precedence of an operator
    ///
    /// ### Returns
    /// * `Ok(number)` The result of the operation if it could be evaluated
    /// * `Err(err)` If there was no number left in the input stack at the end
    ///
    /// [Shunting Yard algorithm]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    fn shunting_yard(&self, precedence: impl Fn(Operator) -> u8) -> anyhow::Result<u64> {
        // A closure that pop operators from the stack and compute their result on the input
        // Return an error if an operation went wrong
        let pop_operators =
            |operands: &mut Vec<u64>, operators: &mut Vec<Operator>, min_precedence: u8| {
                while let Some(op) = operators.pop() {
                    if op == Operator::OpenParen || precedence(op) < min_precedence {
                        operators.push(op);
                        break;
                    } else if let Some(res) = op.binary_op(operands.pop().zip(operands.pop())) {
                        operands.push(res);
                    } else {
                        return Err(anyhow::anyhow!("Bad operator call {:?}", op));
                    }
                }

                Ok(())
            };

        let mut operators: Vec<Operator> = Vec::with_capacity(3);
        let mut operands: Vec<u64> = Vec::with_capacity(4);
        for &token in &self.0 {
            match token {
                Token::Number(n) => operands.push(n),
                Token::Operator(Operator::ClosingParen) => {
                    pop_operators(&mut operands, &mut operators, 1)?;
                    operators.pop(); // Remove the left parenthesis that ended the pop_operators
                }
                Token::Operator(Operator::OpenParen) => operators.push(Operator::OpenParen),
                Token::Operator(operator) => {
                    pop_operators(&mut operands, &mut operators, precedence(operator))?;
                    operators.push(operator);
                }
            }
        }

        // Compute all remaining operations in the stack
        pop_operators(&mut operands, &mut operators, 0)?;

        if operands.len() == 1 {
            Ok(operands[0])
        } else {
            Err(anyhow::anyhow!(
                "Not exactly 1 element at the end in the operands: {:?}",
                operands
            ))
        }
    }
}

impl FromStr for Operation {
    type Err = TokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::with_capacity(20);
        let mut current = s.trim_start();
        while !current.is_empty() {
            let (token, rest) = Token::parse(current)?;
            current = rest.trim_start();
            tokens.push(token);
        }

        Ok(Operation(tokens))
    }
}

/// An operator in the shunting-yard algorithm
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Plus,
    Mul,
    OpenParen,
    ClosingParen,
}

impl Operator {
    /// Apply the Operator as a binary operator on the numbers
    ///
    /// This will pop the required quantity of inputs from the numbers and then push the result
    ///
    /// ### Arguments
    /// * `numbers` - The two inputs for the operator
    ///
    /// ### Returns
    /// * `Some(result)` if the operator is a binary operator and the operand are defined
    /// * `None` if the operator is not a binary operator or the operand aren't defined
    fn binary_op(self, numbers: Option<(u64, u64)>) -> Option<u64> {
        match self {
            Operator::Plus => numbers.map(|(a, b)| a + b),
            Operator::Mul => numbers.map(|(a, b)| a * b),
            _ => None,
        }
    }
}

/// A raw token parsed from the operation String (any significant element of it)
#[derive(Debug, Copy, Clone)]
enum Token {
    Operator(Operator),
    Number(u64),
}

impl Token {
    /// Parse the next token in an input, returning the token and the rest of the input
    fn parse(current: &str) -> Result<(Token, &str), TokenParseError> {
        let last_numeric = current
            .char_indices()
            .take_while(|(_, c)| c.is_ascii_digit())
            .map(|(i, _)| i + 1)
            .last();

        if let Some(last_numeric) = last_numeric {
            let (number, rest) = current.split_at(last_numeric);
            match number.parse::<u64>() {
                Ok(number) => Ok((Token::Number(number), rest)),
                Err(err) => Err(TokenParseError::ParseNumberError(number.into(), err)),
            }
        } else {
            match current.chars().next() {
                Some('+') => Ok((Token::Operator(Operator::Plus), &current[1..])),
                Some('*') => Ok((Token::Operator(Operator::Mul), &current[1..])),
                Some('(') => Ok((Token::Operator(Operator::OpenParen), &current[1..])),
                Some(')') => Ok((Token::Operator(Operator::ClosingParen), &current[1..])),
                _ => Err(TokenParseError::UnknownToken(current.into())),
            }
        }
    }
}

/// An error that can happen while lexing the operations
#[derive(Debug, thiserror::Error)]
pub enum TokenParseError {
    #[error("Expected token to be number, +, *, ( or ), but was {0}")]
    UnknownToken(Box<str>),
    #[error("Number token parse error {0} (due to {1})")]
    ParseNumberError(Box<str>, #[source] std::num::ParseIntError),
}

#[cfg(test)]
mod tests;
