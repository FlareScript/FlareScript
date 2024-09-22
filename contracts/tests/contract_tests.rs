// tests/contract_tests.rs

use flarescript_contracts::lexer::tokenize;
use flarescript_contracts::parser::parse;
use flarescript_contracts::compiler::compile;
use flarescript_contracts::ast::{ASTNode, ExpressionNode, StatementNode};

#[test]
fn test_lexer() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    assert!(tokenize(contract_code).is_ok());
}

#[test]
fn test_parser() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    let tokens = tokenize(contract_code).unwrap();
    let ast = parse(tokens);

    let ASTNode::Program { contracts } = ast;

    assert_eq!(contracts.len(), 1);
    let contract = &contracts[0];
    assert_eq!(contract.name, "Token");
    assert_eq!(contract.functions.len(), 1);

    let function = &contract.functions[0];
    assert_eq!(function.name, "transfer");
    assert_eq!(function.parameters, vec!["wallet1", "wallet2", "amount"]);
    assert_eq!(function.body.len(), 1);

    if let StatementNode::FunctionCall {
        receiver,
        function_name,
        arguments,
    } = &function.body[0]
    {
        assert_eq!(receiver.as_ref().unwrap(), "Token");
        assert_eq!(function_name, "transfer");
        assert_eq!(arguments.len(), 3);

        let expected_args = vec![
            ExpressionNode::Identifier("wallet1".to_string()),
            ExpressionNode::Identifier("wallet2".to_string()),
            ExpressionNode::Identifier("amount".to_string()),
        ];

        assert_eq!(arguments, &expected_args);
    } else {
        panic!("Expected FunctionCall statement");
    }
}

#[test]
fn test_compiler_evm() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
            // Simulate an assignment
            balance = balance - amount;
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    let tokens = tokenize(contract_code).unwrap();
    let ast = parse(tokens);
    let bytecode = compile(ast, "evm");

    assert!(!bytecode.is_empty());
    assert_eq!(bytecode, vec![0x60, 0x60, 0x60, 0x40, 0x52]);
}

#[test]
fn test_compiler_wasm() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
            // Simulate an assignment
            balance = balance - amount;
            Token.transfer(wallet1, wallet2, amount);
        }
    }
    "#;

    let tokens = tokenize(contract_code).unwrap();
    let ast = parse(tokens);
    let wasm_bytecode = compile(ast, "wasm");

    assert!(!wasm_bytecode.is_empty());
    assert_eq!(wasm_bytecode, vec![0x00, 0x61, 0x73, 0x6D]);
}
