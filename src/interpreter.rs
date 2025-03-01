use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::parser::*;

impl Expression {
    fn evaluate(&self, variables: &HashMap<String, VariableValue>) -> VariableValue {
        match self {
            Expression::Term(term) => term.evaluate(variables),
            Expression::Operation { left, op, right } => {
                let left = left.evaluate(variables);
                let right = right.evaluate(variables);

                match op {
                    Operator::Add => left + right,
                    Operator::Subtract => left - right,
                    Operator::Multiply => left * right,
                    Operator::Divide => left / right,
                }
            }
        }
    }
}

impl Term {
    fn evaluate(&self, variables: &HashMap<String, VariableValue>) -> VariableValue {
        match self {
            Term::Number(number) => VariableValue::Integer(*number),
            Term::String(string) => VariableValue::String(string.clone()),
            Term::Variable(variable) => {
                let Some(value) = variables.get(variable) else {
                    panic!("Kunde inte hitta variabeln \"{}\"", variable);
                };

                match value {
                    VariableValue::Integer(number) => VariableValue::Integer(*number),
                    VariableValue::String(string) => VariableValue::String(string.clone()),
                }
            }
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, VariableValue>,
}

pub enum VariableValue {
    Integer(i64),
    String(String),
}

impl Add for VariableValue {
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a + b),
            (Self::Integer(a), Self::String(b)) => Self::String(format!("{}{}", a, b)),
            (Self::String(a), Self::Integer(b)) => Self::String(format!("{}{}", a, b)),
            (Self::String(a), Self::String(b)) => Self::String(format!("{}{}", a, b)),
        }
    }

    type Output = Self;
}

impl Sub for VariableValue {
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a - b),
            _ => panic!("Kan inte subtrahera olika typer av värden"),
        }
    }

    type Output = Self;
}

impl Mul for VariableValue {
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a * b),
            _ => panic!("Kan inte multiplicera olika typer av värden"),
        }
    }

    type Output = Self;
}

impl Div for VariableValue {
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                if b == 0 {
                    panic!("Kan inte dela med noll!")
                }
                Self::Integer(a / b)
            }
            _ => panic!("Kan inte dividera olika typer av värden"),
        }
    }

    type Output = Self;
}

impl Display for VariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VariableValue::Integer(number) => write!(f, "{}", number),
            VariableValue::String(string) => write!(f, "{}", string),
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        statements.iter().for_each(|statement| match statement {
            Statement::Assignment {
                variable,
                expression,
            } => match variable {
                Term::Variable(identifier) => {
                    self.variables
                        .insert(identifier.clone(), expression.evaluate(&self.variables));
                }
                _ => {
                    panic!("Expected variable identifier");
                }
            },
            Statement::Print { expression } => {
                println!("{}", expression.evaluate(&self.variables));
            }
            Statement::If {
                if_expression,
                if_statements,
                else_statements,
            } => {
                let if_value = if_expression.evaluate(&self.variables);

                let if_true = match if_value {
                    VariableValue::Integer(number) => number > 0,
                    VariableValue::String(string) => !string.is_empty(),
                };

                if if_true {
                    self.interpret(if_statements.clone());
                } else {
                    self.interpret(else_statements.clone());
                }
            }
        });
    }
}
