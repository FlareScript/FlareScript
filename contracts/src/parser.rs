// src/parser.rs

use crate::lexer::Rule;
use crate::ast::*;
use pest::iterators::{Pair, Pairs};

pub fn parse(mut pairs: Pairs<Rule>) -> ASTNode {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::program => parse_program(pair),
        _ => panic!("Expected program"),
    }
}

fn parse_program(pair: Pair<Rule>) -> ASTNode {
    let mut contracts = vec![];

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::contract => {
                let contract_node = parse_contract(inner_pair);
                contracts.push(contract_node);
            }
            Rule::EOI => break, // End of input
            _ => {}
        }
    }

    ASTNode::Program { contracts }
}

fn parse_contract(pair: Pair<Rule>) -> ContractNode {
    let mut inner = pair.into_inner();
    let name_pair = inner.next().unwrap(); // Contract name
    let name = name_pair.as_str().to_string();
    let mut functions = vec![];

    for inner_pair in inner {
        match inner_pair.as_rule() {
            Rule::function => {
                let function_node = parse_function(inner_pair);
                functions.push(function_node);
            }
            Rule::WHITESPACE | Rule::COMMENT => {
                // Skip whitespace and comments
            }
            _ => {}
        }
    }

    ContractNode { name, functions }
}

fn parse_function(pair: Pair<Rule>) -> FunctionNode {
    let mut inner = pair.into_inner();
    let name_pair = inner.next().unwrap(); // Function name
    let name = name_pair.as_str().to_string();
    let params_pair = inner.next().unwrap();
    let parameters = parse_parameter_list(params_pair);

    // Handle optional return type
    let return_type = if let Some(next_pair) = inner.peek() {
        if next_pair.as_rule() == Rule::type_name {
            Some(inner.next().unwrap().as_str().to_string())
        } else {
            None
        }
    } else {
        None
    };

    let body_pair = inner.next().unwrap();
    let body = parse_block(body_pair);

    FunctionNode {
        name,
        parameters,
        return_type,
        body,
    }
}

fn parse_parameter_list(pair: Pair<Rule>) -> Vec<ParameterNode> {
    let mut parameters = vec![];

    for param_pair in pair.into_inner() {
        match param_pair.as_rule() {
            Rule::parameter => {
                let mut name = String::new();
                let mut type_name = String::new();

                for inner_pair in param_pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::ident => {
                            name = inner_pair.as_str().to_string();
                        }
                        Rule::type_name => {
                            type_name = inner_pair.as_str().to_string();
                        }
                        _ => {}
                    }
                }

                parameters.push(ParameterNode { name, type_name });
            }
            Rule::COMMA => {
                // Skip commas
            }
            _ => {}
        }
    }

    parameters
}

fn parse_block(pair: Pair<Rule>) -> Vec<StatementNode> {
    let mut statements = vec![];

    for statement_pair in pair.into_inner() {
        match statement_pair.as_rule() {
            Rule::statement => {
                let statement = parse_statement(statement_pair);
                statements.push(statement);
            }
            Rule::WHITESPACE | Rule::COMMENT => {
                // Skip whitespace and comments
            }
            _ => {}
        }
    }

    statements
}

fn parse_statement(pair: Pair<Rule>) -> StatementNode {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::assignment => parse_assignment(inner_pair),
        Rule::function_call => parse_function_call(inner_pair),
        Rule::if_statement => parse_if_statement(inner_pair),
        Rule::while_statement => parse_while_statement(inner_pair),
        Rule::for_statement => parse_for_statement(inner_pair),
        Rule::variable_declaration => parse_variable_declaration(inner_pair),
        Rule::return_statement => parse_return_statement(inner_pair),
        _ => panic!("Unknown statement: {:?}", inner_pair.as_rule()),
    }
}

fn parse_assignment(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    let variable_pair = inner.next().unwrap();
    let variable = variable_pair.as_str().to_string();
    inner.next(); // Consume '='
    let value_pair = inner.next().unwrap();
    let value = parse_expression(value_pair);

    StatementNode::Assignment { variable, value }
}

fn parse_function_call(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    let ident_pair = inner.next().unwrap();
    let next_pair = inner.peek();

    let (receiver, function_name) = if let Some(next_pair) = next_pair {
        if next_pair.as_rule() == Rule::dot {
            inner.next(); // Consume '.'
            let method_name_pair = inner.next().unwrap();
            (
                Some(ident_pair.as_str().to_string()),
                method_name_pair.as_str().to_string(),
            )
        } else {
            (None, ident_pair.as_str().to_string())
        }
    } else {
        (None, ident_pair.as_str().to_string())
    };

    let args_pair = inner.next().unwrap();
    let arguments = parse_argument_list(args_pair);

    StatementNode::FunctionCall {
        receiver,
        function_name,
        arguments,
    }
}

fn parse_argument_list(pair: Pair<Rule>) -> Vec<ExpressionNode> {
    let mut arguments = vec![];

    for expr_pair in pair.into_inner() {
        match expr_pair.as_rule() {
            Rule::expression => {
                let expr = parse_expression(expr_pair);
                arguments.push(expr);
            }
            Rule::COMMA => {
                // Skip commas
            }
            _ => {}
        }
    }

    arguments
}

fn parse_variable_declaration(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    inner.next(); // Consume 'let'
    let name_pair = inner.next().unwrap();
    let name = name_pair.as_str().to_string();
    inner.next(); // Consume ':'
    let type_pair = inner.next().unwrap();
    let type_name = type_pair.as_str().to_string();

    let initial_value = if let Some(eq_pair) = inner.next() {
        if eq_pair.as_str() == "=" {
            let value_pair = inner.next().unwrap();
            Some(parse_expression(value_pair))
        } else {
            None
        }
    } else {
        None
    };

    StatementNode::VariableDeclaration {
        name,
        type_name,
        initial_value,
    }
}

fn parse_if_statement(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    inner.next(); // Consume 'if'
    let condition_pair = inner.next().unwrap();
    let condition = parse_expression(condition_pair);
    let then_block_pair = inner.next().unwrap();
    let then_branch = parse_block(then_block_pair);

    let else_branch = if let Some(else_clause_pair) = inner.next() {
        match else_clause_pair.as_rule() {
            Rule::else_clause => {
                let else_inner = else_clause_pair.into_inner().next().unwrap();
                match else_inner.as_rule() {
                    Rule::if_statement => Some(vec![parse_if_statement(else_inner)]),
                    Rule::block => Some(parse_block(else_inner)),
                    _ => panic!("Unknown else clause"),
                }
            }
            _ => None,
        }
    } else {
        None
    };

    StatementNode::IfStatement {
        condition,
        then_branch,
        else_branch,
    }
}

fn parse_while_statement(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    inner.next(); // Consume 'while'
    let condition_pair = inner.next().unwrap();
    let condition = parse_expression(condition_pair);
    let body_pair = inner.next().unwrap();
    let body = parse_block(body_pair);

    StatementNode::WhileStatement { condition, body }
}

fn parse_for_statement(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    inner.next(); // Consume 'for'
    let init_pair = inner.next();
    let initializer = if let Some(init_pair) = init_pair {
        if init_pair.as_rule() == Rule::for_init {
            let init_inner = init_pair.into_inner().next();
            init_inner.map(|pair| Box::new(parse_statement(pair)))
        } else {
            None
        }
    } else {
        None
    };

    let condition_pair = inner.next();
    let condition = condition_pair.map(|pair| parse_expression(pair));

    let update_pair = inner.next();
    let update = if let Some(update_pair) = update_pair {
        if update_pair.as_rule() == Rule::for_update {
            let update_inner = update_pair.into_inner().next();
            update_inner.map(|pair| Box::new(parse_statement(pair)))
        } else {
            None
        }
    } else {
        None
    };

    let body_pair = inner.next().unwrap();
    let body = parse_block(body_pair);

    StatementNode::ForStatement {
        initializer,
        condition,
        update,
        body,
    }
}

fn parse_return_statement(pair: Pair<Rule>) -> StatementNode {
    let mut inner = pair.into_inner();
    inner.next(); // Consume 'return'
    let value = if let Some(expr_pair) = inner.next() {
        Some(parse_expression(expr_pair))
    } else {
        None
    };

    StatementNode::ReturnStatement { value }
}

fn parse_expression(pair: Pair<Rule>) -> ExpressionNode {
    match pair.as_rule() {
        Rule::addition => parse_addition(pair),
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::number => ExpressionNode::Number(pair.as_str().to_string()),
        Rule::ident => ExpressionNode::Identifier(pair.as_str().to_string()),
        Rule::string => {
            let string_value = pair.as_str().to_string();
            let unquoted = string_value[1..string_value.len() - 1].to_string();
            ExpressionNode::StringLiteral(unquoted)
        }
        _ => panic!("Unknown expression: {:?}", pair.as_rule()),
    }
}

fn parse_addition(pair: Pair<Rule>) -> ExpressionNode {
    let mut inner = pair.into_inner();
    let mut left = parse_multiplication(inner.next().unwrap());

    while let Some(operator_pair) = inner.next() {
        let operator = operator_pair.as_str().to_string();
        let right = parse_multiplication(inner.next().unwrap());

        left = ExpressionNode::BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    left
}

fn parse_multiplication(pair: Pair<Rule>) -> ExpressionNode {
    let mut inner = pair.into_inner();
    let mut left = parse_unary(inner.next().unwrap());

    while let Some(operator_pair) = inner.next() {
        let operator = operator_pair.as_str().to_string();
        let right = parse_unary(inner.next().unwrap());

        left = ExpressionNode::BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    left
}

fn parse_unary(pair: Pair<Rule>) -> ExpressionNode {
    let mut inner = pair.into_inner();
    let mut operators = vec![];

    while let Some(op_pair) = inner.peek() {
        if op_pair.as_rule() == Rule::operator_unary {
            operators.push(op_pair.as_str().to_string());
            inner.next(); // Consume the operator
        } else {
            break;
        }
    }

    let primary_expr = parse_primary(inner.next().unwrap());

    // Apply unary operators in reverse order
    operators.into_iter().rev().fold(primary_expr, |expr, op| {
        ExpressionNode::UnaryOperation {
            operator: op,
            operand: Box::new(expr),
        }
    })
}

fn parse_primary(pair: Pair<Rule>) -> ExpressionNode {
    match pair.as_rule() {
        Rule::number => ExpressionNode::Number(pair.as_str().to_string()),
        Rule::ident => ExpressionNode::Identifier(pair.as_str().to_string()),
        Rule::string => {
            let string_value = pair.as_str().to_string();
            // Remove the surrounding quotes
            let unquoted = string_value[1..string_value.len() - 1].to_string();
            ExpressionNode::StringLiteral(unquoted)
        }
        Rule::expression => {
            let inner_pair = pair.into_inner().next().unwrap();
            parse_expression(inner_pair)
        }
        Rule::primary => {
            let mut inner = pair.into_inner();
            let inner_pair = inner.next().unwrap();
            parse_primary(inner_pair)
        }
        _ => panic!("Unknown primary expression: {:?}", pair.as_rule()),
    }
}
