use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{multispace0, multispace1, space1},
    combinator::{cut, map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
    IResult, Parser as NomParser,
};

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        variable: Term,
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
    If {
        if_expression: Expression,
        if_statements: Vec<Statement>,
        // else_if_expression: Expression,
        // else_if_statement: Box<Statement>,
        else_statements: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Term(Term),
    Operation {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Term {
    Number(i64),
    String(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

const NUMBERS: [&str; 11] = [
    "noll", "ett", "två", "tre", "fyra", "fem", "sex", "sju", "åtta", "nio", "tio",
];

pub fn parse(source_code: &str) -> Result<Vec<Statement>, nom::Err<nom::error::Error<&str>>> {
    // TODO: use nom::terminated here instead of map
    let (_, statements) = many1(map(
        (parse_statement, cut(tag(".")), multispace0),
        |(statement, _, _)| statement,
    ))
    .parse(source_code)?;

    Ok(statements)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let (input, output) = alt((parse_print, parse_assignment, parse_if)).parse(input)?;

    Ok((input, output))
}

fn parse_print(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag_no_case("skriv")(input)?;
    let (input, _) = space1(input)?;
    let (input, expression) = parse_expression(input)?;

    Ok((input, Statement::Print { expression }))
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag_no_case("spara").parse(input)?;
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

fn parse_if(input: &str) -> IResult<&str, Statement> {
    // if expression
    let (input, _) = tag_no_case("om")(input)?;
    let (input, _) = space1(input)?;
    let (input, if_expression) = parse_expression(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = opt(multispace1).parse(input)?;

    // if statements
    let (input, if_statements) = many1(map(
        (
            parse_statement,
            alt((tag(","), tag(" och"))),
            opt(multispace1),
        ),
        |(statement, _, _)| statement,
    ))
    .parse(input)?;

    // else statement
    let (input, _) = tag("annars")(input)?;
    let (input, _) = space1(input)?;

    let (input, else_statements) = separated_list1(
        delimited(
            opt(multispace1),
            alt((tag(","), tag("och"))),
            opt(multispace1),
        ),
        parse_statement,
    )
    .parse(input)?;

    Ok((
        input,
        Statement::If {
            if_expression,
            if_statements,
            else_statements,
        },
    ))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_add_sub(input)
}

fn parse_add_sub(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_mul_div(input)?;

    let (input, op) = opt((space1, alt((tag("plus"), tag("minus"))))).parse(input)?;

    if let Some((_, op)) = op {
        let operator = match op {
            "plus" => Operator::Add,
            "minus" => Operator::Subtract,
            _ => unreachable!(),
        };

        let (input, _) = space1(input)?;
        let (input, right) = parse_add_sub(input)?;

        Ok((
            input,
            Expression::Operation {
                left: Box::new(left),
                op: operator,
                right: Box::new(right),
            },
        ))
    } else {
        Ok((input, left))
    }
}

fn parse_mul_div(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_term(input)?;
    let (input, op) = opt((space1, alt((tag("gånger"), tag("delat med"))))).parse(input)?;

    if let Some((_, op)) = op {
        let operator = match op {
            "gånger" => Operator::Multiply,
            "delat med" => Operator::Divide,
            _ => unreachable!(),
        };

        let (input, _) = space1(input)?;
        let (input, right) = parse_mul_div(input)?;

        Ok((
            input,
            Expression::Operation {
                left: Box::new(Expression::Term(left)),
                op: operator,
                right: Box::new(right),
            },
        ))
    } else {
        Ok((input, Expression::Term(left)))
    }
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_number, parse_string, parse_variable)).parse(input)
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

fn parse_string(input: &str) -> IResult<&str, Term> {
    let (input, output) =
        delimited(tag("\""), take_while1(|c: char| c != '"'), tag("\"")).parse(input)?;

    Ok((input, Term::String(output.to_owned())))
}

fn parse_variable(input: &str) -> IResult<&str, Term> {
    let (input, identifier) = take_while1(|c: char| c.is_alphabetic()).parse(input)?;

    Ok((input, Term::Variable(identifier.to_string())))
}
