// src/main.rs

use flarescript_contracts::lexer::tokenize;
use flarescript_contracts::parser::parse;

fn main() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
            // Simulate an assignment
            balance = balance - amount;
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    match tokenize(contract_code) {
        Ok(tokens) => {
            let ast = parse(tokens);
            println!("{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Tokenization error: {:?}", e);
        }
    }
}
