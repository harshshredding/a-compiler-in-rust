use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

use crate::semantic_graph::Edge;
use super::semantic_analysis::*;
use crate::lexical_analysis::Token;
use crate::lexical_analysis::TokenType;

#[cfg(test)]

#[test]
fn test_semantic_action() {
    use crate::lexical_analysis::Token;

    let production_element = ProductionElement::SemanticElement(
        Box::new(PushHigherLevelNode{description: "+".to_string()})
    );
    let mut stack : Vec<SemanticNode> = vec![];
    let mut all_nodes: Vec<SemanticNode> = vec![];
    let mut edges: Vec<(SemanticNode, SemanticNode)> = vec![];
    let curr_token = Token::from(TokenType::Plus, "plus".to_string());
    match production_element {
        ProductionElement::SemanticElement(handler) =>  {
            handler.take_action(&mut stack, &mut all_nodes, &mut edges, Some(&curr_token));
        },
        _ => {
            panic!("Wrong production type!");
        }
    }
    assert_eq!(stack.len(), 1);
    assert_eq!(all_nodes.len(), 1);
    match stack.pop().unwrap() {
        SemanticNode::HigherLevelNode { id, description } => {
            assert_eq!(id, 0);
            assert_eq!(description, "+")
        }
        _ => {
            panic!("expected higher level node");
        }
    }
}

#[test]
fn test_string_matching() {
    let x = "value1";
    match x {
        "value0" => panic!(),
        "value1" => print!("success"),
        _ => panic!()
    }
}

#[test]
fn test_unicode() {
    let x = "X → Y";
    assert_eq!(x, "X \u{2192} Y")
}


#[test]
fn for_loops_work() {
    let len = 10;
    let mut nums = vec![];
    for i in 0..len {
        nums.push(i);
    }
    assert_eq!(nums.len(), len);
}

#[test]
fn test_semanctic_node() {
    let node = SemanticNode::Identifier { id: 1, symbol: "x".to_string() };
    assert_eq!(node.as_string(), "Id_x_ID1");
}


#[test]
pub fn test_create_adjacency_matrix() {
    let edges: Vec<(SemanticNode, SemanticNode)> = vec![];
    let mut adjacency_matrix: HashMap<SemanticNode, Vec<SemanticNode>> = HashMap::new();
    let mut all_semantic_nodes: Vec<SemanticNode> = vec![];
    let node1 = SemanticNode::new_higher_level_node(&mut all_semantic_nodes, "node1".to_string());
    adjacency_matrix.insert(node1, vec![]);
    assert_eq!(adjacency_matrix.len(), 1);
}
