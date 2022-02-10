use crate::lib::string_parser::parse_string;
use crate::lisp::{Literal, LanguageType};
use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::opt,
    multi::fold_many0,
    sequence::{delimited, terminated, tuple},
    IResult,
};

/// Parse a (possibly negative) number
fn parse_number(input: &str) -> IResult<&str, LanguageType> {
    // TODO parse floating point numbers
    let (input, number) = tuple((opt(char('-')), digit1))(input)?;
    let (_, value) = number;

    let mut num = i64::from_str_radix(value, 10).unwrap();
    if number.0.is_some() {
        num = num * -1;
    }

    Ok((input, LanguageType::LiteralValue(Literal::Number(num))))
}

/// Parse a literal value, either a string or a number
fn parse_literal(input: &str) -> IResult<&str, LanguageType> {
    let (input, literal) = alt((parse_string, parse_number))(input)?;
    Ok((input, literal))
}

/// Parse a symbol/identifier
fn parse_symbol(input: &str) -> IResult<&str, LanguageType> {
    let (input, symbol) = alpha1(input)?;
    Ok((input, LanguageType::Symbol(symbol.to_string())))
}

/// Parse a bunch of heterogenous values
fn parse_many_vals(input: &str) -> IResult<&str, LanguageType> {
    let (input, values) = fold_many0(
        terminated(alt((parse_literal, parse_symbol, parse_funcall, parse_collection)), multispace0),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(input)
    .unwrap();

    Ok((input, LanguageType::ArgList(values)))
}

/// Parse heterogenous collection of values inside of a collection
fn parse_collection(input: &str) -> IResult<&str, LanguageType> {
    delimited(char('['), parse_many_vals, char(']'))(input)
}

/// Parse a function call
pub fn parse_funcall(input: &str) -> IResult<&str, LanguageType> {
    let (input, funcall) =
        tuple((char('('), parse_symbol, opt(tuple((multispace0, parse_many_vals))), char(')')))(input)?;
    let (_, symbol, args, _) = funcall;

    if let Some(list) = args {
        let (_, args) = list;
        return Ok((input, LanguageType::Function{name: Box::new(symbol), args: Box::new(args)}));
    } else {
        return Ok((input, LanguageType::Function{name: Box::new(symbol), args: Box::new(LanguageType::Nil)}));
    }
}
