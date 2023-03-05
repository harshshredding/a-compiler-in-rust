use crate::lexical_analysis::TokenType::Plus;
use crate::semantic_analysis::ProductionElement::{SemanticElement, SyntaxElement};
use super::semantic_graph::Edge;

#[derive(Clone, Debug)]
pub struct SemanticNode {
    pub id: usize,
    pub val: String,
}

impl SemanticNode {
    fn as_string(&self) -> String {
        return format!("{}{}", self.val, self.id);
    }
}

pub trait SemanticAction {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<Edge>
    );
}

pub struct CheckStackOneNode;

impl SemanticAction for CheckStackOneNode {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<Edge>
    ) {
        assert_eq!(semantic_stack.len(), 1);
        let root_node = semantic_stack.pop()
            .expect("Should be able to get the last remaining node.");
        edges.push(("START".to_string(), root_node.as_string()));
    }
}

pub struct StoreValue {
    pub value: String,
}

impl SemanticAction for StoreValue {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<Edge>
    ) {
        let new_node = SemanticNode {
            id: all_semantic_nodes.len(),
            val: self.value.clone(),
        };
        semantic_stack.push(
            new_node.clone()
        );
        all_semantic_nodes.push(
            new_node.clone()
        )
    }
}

pub struct PlusGather;

impl SemanticAction for PlusGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<Edge>
    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let symbol = semantic_stack.get(semantic_stack.len() - 2).unwrap().val.clone();
        assert!(
            (&symbol == "Or") || (&symbol == "Minus") || (&symbol == "Plus"),
            "The second element needs to be mult,and,or div. stack {:?}", semantic_stack
        );
        let operand1 = semantic_stack.pop().unwrap();
        let operator = semantic_stack.pop().unwrap();
        let operand2 = semantic_stack.pop().unwrap();
        edges.push((operator.as_string(), operand1.as_string()));
        edges.push((operator.as_string(), operand2.as_string()));
        semantic_stack.push(operator);
    }
}


pub struct MultGather;

impl SemanticAction for MultGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<Edge>
    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let symbol = semantic_stack.get(semantic_stack.len() - 2).unwrap().val.clone();
        assert!(
            (&symbol == "And") || (&symbol == "Div") || (&symbol == "Mult"),
            "The second element needs to be mult,and,or div. stack {:?}", semantic_stack
        );
        let operand1 = semantic_stack.pop().unwrap();
        let operator = semantic_stack.pop().unwrap();
        let operand2 = semantic_stack.pop().unwrap();
        edges.push((operator.as_string(), operand1.as_string()));
        edges.push((operator.as_string(), operand2.as_string()));
        semantic_stack.push(operator);
    }
}

pub enum ProductionElement {
    SyntaxElement(String),
    SemanticElement(Box<dyn SemanticAction>),
}

impl ProductionElement {
    pub fn is_syntax(&self) -> bool {
        return match self {
            SyntaxElement(_) => true,
            _ => false
        }
    }

    pub fn is_semantic(&self) -> bool {
        return !self.is_syntax()
    }
}

pub fn get_only_syntax_elements(production: &str) -> Vec<ProductionElement> {
    let production_parts: Vec<&str> = production.split_whitespace().collect();
    return production_parts[2..].into_iter().map(|x| SyntaxElement(x.to_string())).collect()
}

pub fn get_production_elements(production_string: &String) -> Vec<ProductionElement> {
    match production_string.as_str() {
        "START → ARITHEXPR eof" => {
            return vec![
                SyntaxElement("ARITHEXPR".to_string()),
                SyntaxElement("eof".to_string()),
                SemanticElement(Box::new(CheckStackOneNode)),
            ];
        }
        "ADDTERMS → ADDOP ARITHEXPR" => {
            return vec![
                SyntaxElement(String::from("ADDOP")),
                SyntaxElement(String::from("ARITHEXPR")),
                SemanticElement(Box::new(PlusGather)),
            ];
        }
        "MULTIPLYLITERALS → MULTOP TERM" => {
            return vec![
                SyntaxElement(String::from("MULTOP")),
                SyntaxElement(String::from("TERM")),
                SemanticElement(Box::new(MultGather)),
            ];
        }
        "ADDOP → or" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Or".to_string()})),
                SyntaxElement("or".into()),
            ]
        }
        "ADDOP → minus" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Minus".to_string()})),
                SyntaxElement("minus".into()),
            ]
        }
        "ADDOP → plus" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Plus".to_string()})),
                SyntaxElement("plus".into()),
            ]
        }
        "MULTOP → and" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "And".to_string()})),
                SyntaxElement("and".into()),
            ]
        }
        "MULTOP → div" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Div".to_string()})),
                SyntaxElement("div".into()),
            ]
        }
        "MULTOP → mult" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Mult".to_string()})),
                SyntaxElement("mult".into()),
            ]
        }
        "LITERAL → floatlit" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Float".to_string()})),
                SyntaxElement("floatlit".into()),
            ]
        }
        "LITERAL → intlit" => {
            return vec![
                SemanticElement(Box::new(StoreValue{value: "Int".to_string()})),
                SyntaxElement("intlit".into()),
            ]
        }
        "ADDTERMS → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "MULTIPLYLITERALS → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "ARITHEXPR → TERM ADDTERMS" => {
            return get_only_syntax_elements(production_string)
        }
        "TERM → LITERAL MULTIPLYLITERALS" => {
            return get_only_syntax_elements(production_string)
        }
        _ => panic!("Can't add semantics to ({}) at the moment", production_string.as_str())
    };
}
