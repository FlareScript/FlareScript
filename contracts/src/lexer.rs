// src/lexer.rs

use pest::Parser;

use pest::error::Error;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FlareScriptParser;

pub fn tokenize(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let tokens = FlareScriptParser::parse(Rule::program, input)?;
    Ok(tokens)
}
