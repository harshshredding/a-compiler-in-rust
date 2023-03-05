pub trait SemanticAction {
    fn take_action(&self, semantic_stack: &mut Vec<String>);
}

pub struct CreatePlusNode;

impl SemanticAction for CreatePlusNode {
    fn take_action(&self, semantic_stack: &mut Vec<String>) {
        semantic_stack.push("+".to_string())
    }
}

pub enum Production <A>
where A : SemanticAction {
    NonTerminal(String),
    SemanticProduction(A)
}