use flarescript_contracts::lexer::tokenize;
use flarescript_contracts::parser::parse;
use flarescript_contracts::compiler::compile;

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
    assert_eq!(ast.node_type, "Contract");
}

#[test]
fn test_compiler_evm() {
    let contract_code = r#"
    contract Token {
        function transfer(wallet1, wallet2, amount) {
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
