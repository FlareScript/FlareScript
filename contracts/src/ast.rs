// src/ast.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ASTNode {
    Program {
        contracts: Vec<ContractNode>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContractNode {
    pub name: String,
    pub functions: Vec<FunctionNode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<ParameterNode>,
    pub return_type: Option<String>,
    pub body: Vec<StatementNode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParameterNode {
    pub name: String,
    pub type_name: String,
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
    IfStatement {
        condition: ExpressionNode,
        then_branch: Vec<StatementNode>,
        else_branch: Option<Vec<StatementNode>>,
    },
    WhileStatement {
        condition: ExpressionNode,
        body: Vec<StatementNode>,
    },
    ForStatement {
        initializer: Option<Box<StatementNode>>,
        condition: Option<ExpressionNode>,
        update: Option<Box<StatementNode>>,
        body: Vec<StatementNode>,
    },
    VariableDeclaration {
        name: String,
        type_name: String,
        initial_value: Option<ExpressionNode>,
    },
    ReturnStatement {
        value: Option<ExpressionNode>,
    },
    // Add other statement types as needed
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
    // Add other expression types as needed
}
