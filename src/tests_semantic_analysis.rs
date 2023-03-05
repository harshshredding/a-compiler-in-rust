use crate::semantic_graph::Edge;
use super::semantic_analysis::*;

#[cfg(test)]

#[test]
fn test_semantic_action() {
    let production_element = ProductionElement::SemanticElement(
        Box::new(StoreValue{value: "+".to_string()})
    );
    let mut stack : Vec<SemanticNode> = vec![];
    let mut all_nodes: Vec<SemanticNode> = vec![];
    let mut edges: Vec<Edge> = vec![];
    match production_element {
        ProductionElement::SemanticElement(handler) =>  {
            handler.take_action(&mut stack, &mut all_nodes, &mut edges);
        },
        _ => {
            panic!("Wrong production type!");
        }
    }
    assert_eq!(stack.len(), 1);
    assert_eq!(all_nodes.len(), 1);
    match stack.pop().unwrap() {
        SemanticNode{id, val} => {
            assert_eq!(id, 0);
            assert_eq!(val, "+")
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