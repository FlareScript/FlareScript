// src/compiler.rs

use crate::evm_backend::compile_to_evm;
use crate::wasm_backend::compile_to_wasm_internal;
use crate::ast::ASTNode;  // Import ASTNode from the ast module

pub fn compile(ast: ASTNode, target: &str) -> Vec<u8> {
    match target {
        "evm" => compile_to_evm(ast),
        "wasm" => {
            // Use the internal function for testing
            compile_to_wasm_internal(&ast)
        },
        _ => panic!("Unknown target!"),
    }
}
