// src/ast.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ASTNode {
    Program {
        contracts: Vec<ContractNode>,
    },
    // Other variants can be added if needed
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContractNode {
    pub name: String,
    pub functions: Vec<FunctionNode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<StatementNode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StatementNode {
    Assignment {
        variable: String,
        value: ExpressionNode,
    },
    FunctionCall {
        receiver: Option<String>,
        function_name: String,
        arguments: Vec<ExpressionNode>,
    },
    // Additional statement types can be added here
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ExpressionNode {
    Number(String),
    Identifier(String),
    StringLiteral(String),
    BinaryOperation {
        left: Box<ExpressionNode>,
        operator: String,
        right: Box<ExpressionNode>,
    },
    UnaryOperation {
        operator: String,
        operand: Box<ExpressionNode>,
    },
    // Additional expression types can be added here
}
