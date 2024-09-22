// tests/contract_tests.rs

use flarescript_contracts::lexer::tokenize;
use flarescript_contracts::parser::parse;
use flarescript_contracts::ast::ASTNode;

#[test]
fn test_function_parameters() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1: Address, wallet2: Address, amount: uint256) {
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    let tokens = tokenize(contract_code).unwrap();
    let ast = parse(tokens);

    let ASTNode::Program { contracts } = ast;

    let contract = &contracts[0];
    let function = &contract.functions[0];

    // Extract the names of the parameters from ParameterNode
    let param_names: Vec<String> = function
        .parameters
        .iter()
        .map(|param| param.name.clone())
        .collect();

    assert_eq!(param_names, vec!["wallet1", "wallet2", "amount"]);
}
