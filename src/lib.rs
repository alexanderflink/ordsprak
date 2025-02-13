use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{alpha1, multispace0, newline, space0, space1},
    combinator::opt,
    multi::{many0, many1},
    sequence::{delimited, tuple},
    Err, IResult, Parser,
};

#[derive(Debug)]
pub enum Statement {
    Assignment {
        variable: Term,
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    Operation {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum BuiltIn {
    Print,
}

#[derive(Debug)]
pub enum Term {
    Number(i64),
    String(String),
    Variable(Variable),
}

#[derive(Debug)]
pub enum Variable {
    String(String),
    Number(i64),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

const NUMBERS: [&str; 11] = [
    "noll", "ett", "två", "tre", "fyra", "fem", "sex", "sju", "åtta", "nio", "tio",
];

pub fn run(source_code: &str) -> Result<(), nom::Err<nom::error::Error<&str>>> {
    let (input, output) = many1(parse_statement).parse(source_code)?;

    println!("{}", source_code);
    println!("input: {:?}", input);
    println!("output: {:?}", output);

    Ok(())
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let (input, output) = alt((parse_print, parse_assignment)).parse(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, output))
}

fn parse_print(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("Skriv")(input)?;
    let (input, _) = space1(input)?;
    let (input, expression) = parse_expression(input)?;

    Ok((input, Statement::Print { expression }))
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("Spara").parse(input)?;
    let (input, _) = space1(input)?;
    let (input, expression) = parse_expression(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("i")(input)?;
    let (input, _) = space1(input)?;
    let (input, variable) = parse_variable(input)?;
    Ok((
        input,
        Statement::Assignment {
            variable,
            expression,
        },
    ))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_term.parse(input)?;
    let (input, op) = opt((space1, parse_operator)).parse(input)?;
    if let Some((_, op)) = op {
        let (input, _) = space1(input)?;
        let (input, right) = parse_expression(input)?;

        Ok((
            input,
            Expression::Operation {
                left: Box::new(Expression::Term(left)),
                op,
                right: Box::new(right),
            },
        ))
    } else {
        Ok((input, Expression::Term(left)))
    }
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let (input, _) = tag_no_case("plus").parse(input)?;

    Ok((input, Operator::Plus))
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_number, parse_variable)).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Term> {
    let (input, output) = alt(NUMBERS.map(|n| tag(n))).parse(input)?;

    let number = NUMBERS
        .iter()
        .position(|&n| n == output)
        .map(|n| n as i64)
        .ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))?;

    Ok((input, Term::Number(number)))
}

fn parse_variable(input: &str) -> IResult<&str, Term> {
    let (input, identifier) = delimited(
        tag("'"),
        take_while1(|c: char| c.is_alphabetic() || c == ' '),
        tag("'"),
    )
    .parse(input)?;

    Ok((
        input,
        Term::Variable(Variable::String(identifier.to_string())),
    ))
}
