// src/ast.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ASTNode {
    pub node_type: String,
    pub body: Vec<ASTNode>,
}
