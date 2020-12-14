use crate::DEFAULT_PRECISION;
use ansi_term::Colour::Red;
use kalk::parser::{self, CalcError, CalcError::*};

pub fn eval(parser: &mut parser::Context, input: &str, precision: u32) {
    match parser::eval(parser, input, precision) {
        Ok(Some(result)) => {
            let sci_notation = result.to_scientific_notation();
            let result_str = if sci_notation.exponent > 8 || sci_notation.exponent < -6 {
                sci_notation.to_string()
            } else if precision == DEFAULT_PRECISION {
                result.to_string()
            } else {
                result.to_string_big()
            };

            println!("{} {}", result_str, result.get_unit());
        }
        Ok(None) => print!(""),
        Err(err) => print_calc_err(err),
    }
}

pub fn print_err(msg: &str) {
    Red.paint(msg).to_string();
    println!("{}", msg);
}

fn print_calc_err(err: CalcError) {
    print_err(&match err {
        IncorrectAmountOfArguments(expected, func, got) => format!(
            "Expected {} arguments for function {}, but got {}.",
            expected, func, got
        ),
        InvalidNumberLiteral(x) => format!("Invalid number literal: '{}'.", x),
        InvalidOperator => format!("Invalid operator."),
        InvalidUnit => format!("Invalid unit."),
        UnexpectedToken(got, expected) => {
            format!("Unexpected token: '{:?}', expected '{:?}'.", got, expected)
        }
        UnableToInvert(msg) => format!("Unable to invert: {}", msg),
        UndefinedFn(name) => format!("Undefined function: '{}'.", name),
        UndefinedVar(name) => format!("Undefined variable: '{}'.", name),
        UnableToParseExpression => format!("Unable to parse expression."),
        Unknown => format!("Unknown error."),
    });
}
