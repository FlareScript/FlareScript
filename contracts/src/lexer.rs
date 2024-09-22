use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FlareScriptParser;

pub fn tokenize(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let tokens = FlareScriptParser::parse(Rule::program, input)?;
    Ok(tokens)
}
