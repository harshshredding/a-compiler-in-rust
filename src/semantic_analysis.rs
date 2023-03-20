use crate::lexical_analysis::Token;
use crate::lexical_analysis::TokenType;
use crate::semantic_analysis::ProductionElement::{SemanticElement, SyntaxElement};
use super::semantic_graph::Edge;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SemanticNode {
    Identifier{id: usize, symbol: String},
    Type{id: usize, type_string: String},
    ArraySize{id: usize, size: usize},
    HigherLevelNode{id: usize, description: String},
    Marker{id: usize}
}

impl SemanticNode {
    pub fn as_string(&self) -> String {
        match self {
            SemanticNode::Identifier { id, symbol } => {
                return format!("Id_{}_ID{}", symbol, id);
            }
            SemanticNode::Type { id, type_string } => {
                return format!("Type_{}_ID{}", type_string, id);
            }
            SemanticNode::ArraySize { id, size } => { 
                return format!("Size_{}_ID{}", size, id);
            }
            SemanticNode::HigherLevelNode { id, description } => {
                return format!("{}_ID{}", description, id);
            }
            SemanticNode::Marker { id } => {
                return format!("Marker_ID{}", id);
            }
        }
    }

    pub fn new_identifier(all_semantic_nodes: &Vec<SemanticNode>, symbol: String) -> SemanticNode {
        SemanticNode::Identifier { id: all_semantic_nodes.len(), symbol: symbol }
    }

    pub fn new_type(all_semantic_nodes: &Vec<SemanticNode>, type_string: String) -> SemanticNode {
        SemanticNode::Type { id: all_semantic_nodes.len(), type_string: type_string }
    }

    pub fn new_array_size(all_semantic_nodes: &Vec<SemanticNode>, size: usize) -> SemanticNode {
        SemanticNode::ArraySize { id: all_semantic_nodes.len(), size: size }     
    }

    pub fn new_higher_level_node(all_semantic_nodes: &Vec<SemanticNode>, description: String) -> SemanticNode {
        SemanticNode::HigherLevelNode { id: all_semantic_nodes.len(), description: description } 
    }

    pub fn new_marker_node(all_semantic_nodes: &Vec<SemanticNode>) -> SemanticNode {
        SemanticNode::Marker { id: all_semantic_nodes.len() }     
    }

    pub fn is_marker(&self) -> bool {
        matches!(self, SemanticNode::Marker { id })
    }
}


pub trait SemanticAction {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>
    );
}

pub struct CheckStackOneNode;

impl SemanticAction for CheckStackOneNode {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert_eq!(semantic_stack.len(), 1);
        let root_node = semantic_stack.pop()
            .expect("Should be able to get the last remaining node.");
        let start_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "START".to_string());
        edges.push((start_node, root_node));
    }
}

fn push_new_node(
    new_node: SemanticNode,
    semantic_stack: &mut Vec<SemanticNode>,
    all_semantic_nodes: &mut Vec<SemanticNode>,
) {
    semantic_stack.push(
        new_node.clone()
    );
    all_semantic_nodes.push(
        new_node.clone()
    );
}

pub struct PushHigherLevelNode {
    pub description: String,
}

impl SemanticAction for PushHigherLevelNode {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        let new_node = SemanticNode::new_higher_level_node(&all_semantic_nodes, self.description.to_string());
        push_new_node(new_node, semantic_stack, all_semantic_nodes);
    }
}


pub struct PushIdentifier;

impl SemanticAction for PushIdentifier {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>
    ) {
        let curr_token = curr_token.expect("Expected an identifier token, but none available");
        if matches!(curr_token.token_type, TokenType::Identifier) {
            let new_node = SemanticNode::new_identifier(&all_semantic_nodes, curr_token.lexeme.to_string());
            push_new_node(new_node, semantic_stack, all_semantic_nodes);
        } else {
            panic!("Expected an identifier token but got {:?}", curr_token);
        }
    }
}


pub struct PushType {
    pub type_string: String,
}

impl SemanticAction for PushType {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>
    ) {
        let new_node = SemanticNode::new_type(&all_semantic_nodes, self.type_string.to_string());
        push_new_node(new_node, semantic_stack, all_semantic_nodes);
    }
}


pub struct PushArraySize {
    pub size: usize,
}

impl SemanticAction for PushArraySize {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        let new_node = SemanticNode::new_array_size(&all_semantic_nodes, self.size);
        push_new_node(new_node, semantic_stack, all_semantic_nodes);
    }
}

pub struct PlusGather;

impl SemanticAction for PlusGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let symbol_semanctic_node = semantic_stack.get(semantic_stack.len() - 2).unwrap();
        let symbol = match symbol_semanctic_node { 
            SemanticNode::HigherLevelNode { id, description } => {
                description
            }
            _ => {
                panic!("operation symbol should be a higher level node {:?}", symbol_semanctic_node);
            }
        };
        assert!(
            (symbol == "Or") || (symbol == "Minus") || (symbol == "Plus"),
            "The second element needs to be mult,and,or div. stack {:?}", semantic_stack
        );
        let operand1 = semantic_stack.pop().unwrap();
        let operator = semantic_stack.pop().unwrap();
        let operand2 = semantic_stack.pop().unwrap();
        edges.push((operator.clone(), operand1.clone()));
        edges.push((operator.clone(), operand2.clone()));
        push_new_node(operator, semantic_stack, all_semantic_nodes)
    }
}


pub struct LocalVarGather;

impl SemanticAction for LocalVarGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let array_list_node = semantic_stack.pop().unwrap();
        let type_node = semantic_stack.pop().unwrap();
        let id_node = semantic_stack.pop().unwrap();
        let local_var_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "LocalVarDecl".into());
        edges.push((local_var_node.clone(), id_node.clone()));
        edges.push((local_var_node.clone(), type_node.clone()));
        edges.push((local_var_node.clone(), array_list_node.clone()));
        push_new_node(local_var_node, semantic_stack, all_semantic_nodes);
    }
}


pub struct FunctionGather;

impl SemanticAction for FunctionGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 2,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let local_var_decl_list_node = semantic_stack.pop().unwrap();
        let id_node = semantic_stack.pop().unwrap();
        let function_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "Function".into());
        edges.push((function_node.clone(), id_node.clone()));
        edges.push((function_node.clone(), local_var_decl_list_node.clone()));
        push_new_node(function_node, semantic_stack, all_semantic_nodes)
    }
}


pub struct FunctionGatherFull;

impl SemanticAction for FunctionGatherFull {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack doesn't have enough nodes Stack {:?}", semantic_stack);
        let assign_statement_list_node = semantic_stack.pop().unwrap();
        let local_var_decl_list_node = semantic_stack.pop().unwrap();
        let id_node = semantic_stack.pop().unwrap();
        let function_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "Function".into());
        edges.push((function_node.clone(), id_node.clone()));
        edges.push((function_node.clone(), local_var_decl_list_node.clone()));
        edges.push((function_node.clone(), assign_statement_list_node.clone()));
        push_new_node(function_node, semantic_stack, all_semantic_nodes)
    }
}

pub struct Gather {
    gather_type: String,
    num_nodes_to_gather: usize
}

impl SemanticAction for Gather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        let semantic_stack_initial_size = semantic_stack.len();
        assert!(semantic_stack.len() >= self.num_nodes_to_gather,
                "The semantic stack doesn't have enough nodes for gather {:?} Stack {:?}", self.gather_type.as_str(), semantic_stack);
        let new_node = SemanticNode::new_higher_level_node(all_semantic_nodes, self.gather_type.to_string());
        for _ in 0..self.num_nodes_to_gather {
            let node_to_collect = semantic_stack.pop().unwrap();
            edges.push((new_node.clone(), node_to_collect.clone()));
        }    
        assert_eq!(semantic_stack.len(), semantic_stack_initial_size - self.num_nodes_to_gather);
        push_new_node(new_node, semantic_stack, all_semantic_nodes);
    }
}

pub struct ProgramGather;

impl SemanticAction for ProgramGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>
    ) {
        assert!(semantic_stack.len() >= 1,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let function_list_node = semantic_stack.pop().unwrap();
        let program_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "Program".into());
        edges.push((program_node.clone(), function_list_node.clone()));
        push_new_node(program_node, semantic_stack, all_semantic_nodes);
    }
}


pub struct AssignStatementGather;

impl SemanticAction for AssignStatementGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 2,
                "The semantic stack doesn't have enough elements. Stack {:?}", semantic_stack);
        let righthand_side = semantic_stack.pop().unwrap();
        let left_hand_side = semantic_stack.pop().unwrap();
        let assign_statement_node = SemanticNode::new_higher_level_node(all_semantic_nodes, "AssignStatement".into());
        edges.push((assign_statement_node.clone(), left_hand_side.clone()));
        edges.push((assign_statement_node.clone(), righthand_side.clone()));
        push_new_node(assign_statement_node, semantic_stack, all_semantic_nodes)
    }
}


pub struct MultGather;

impl SemanticAction for MultGather {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        assert!(semantic_stack.len() >= 3,
                "The semantic stack should have the operands and plus. Stack {:?}", semantic_stack);
        let symbol_semanctic_node = semantic_stack.get(semantic_stack.len() - 2).unwrap();
        let symbol = match symbol_semanctic_node { 
            SemanticNode::HigherLevelNode { id, description } => {
                description
            }
            _ => {
                panic!("operation symbol should be a higher level node {:?}", symbol_semanctic_node);
            }
        };
        assert!(
            (symbol == "And") || (symbol == "Div") || (symbol == "Mult"),
            "The second element needs to be mult,and,or div. stack {:?}", semantic_stack
        );
        let operand1 = semantic_stack.pop().unwrap();
        let operator = semantic_stack.pop().unwrap();
        let operand2 = semantic_stack.pop().unwrap();
        edges.push((operator.clone(), operand1.clone()));
        edges.push((operator.clone(), operand2.clone()));
        semantic_stack.push(operator);
    }
}

pub struct MarkListBegin;

impl SemanticAction for MarkListBegin {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        let new_marker_node = SemanticNode::new_marker_node(all_semantic_nodes);
        push_new_node(new_marker_node, semantic_stack, all_semantic_nodes);
    }
}

pub struct CollectList {
    pub list_name: String
}

impl SemanticAction for CollectList {
    fn take_action(
        &self,
        semantic_stack: &mut Vec<SemanticNode>,
        all_semantic_nodes: &mut Vec<SemanticNode>,
        edges: &mut Vec<(SemanticNode, SemanticNode)>,
        curr_token: Option<&Token>

    ) {
        let begin_list_node = semantic_stack.iter().find(|&node| node.is_marker());
        assert!(begin_list_node.is_some(), "To collect a list, a start marker should exist, but it doesn't in stack {:?}", semantic_stack);
        let mut collected_elements: Vec<SemanticNode> = vec![];
        while !semantic_stack.last().unwrap().is_marker() {
            // store the values
            collected_elements.push(semantic_stack.pop().unwrap())
        }
        assert!(semantic_stack.pop().unwrap().is_marker());
        let list_node = SemanticNode::new_higher_level_node(
            all_semantic_nodes,
            format!("{}", self.list_name)
        );
        for collected_item in collected_elements {
            edges.push((list_node.clone(), collected_item.clone()))
        }
        push_new_node(list_node, semantic_stack, all_semantic_nodes);
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
                SemanticElement(Box::new(PushHigherLevelNode{description: "Or".to_string()})),
                SyntaxElement("or".into()),
            ]
        }
        "ADDOP → minus" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Minus".to_string()})),
                SyntaxElement("minus".into()),
            ]
        }
        "ADDOP → plus" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Plus".to_string()})),
                SyntaxElement("plus".into()),
            ]
        }
        "MULTOP → and" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "And".to_string()})),
                SyntaxElement("and".into()),
            ]
        }
        "MULTOP → div" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Div".to_string()})),
                SyntaxElement("div".into()),
            ]
        }
        "MULTOP → mult" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Mult".to_string()})),
                SyntaxElement("mult".into()),
            ]
        }
        "LITERAL → floatlit" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Float".to_string()})),
                SyntaxElement("floatlit".into()),
            ]
        }
        "LITERAL → intlit" => {
            return vec![
                SemanticElement(Box::new(PushHigherLevelNode{description: "Int".to_string()})),
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
        //
        // Local Var Declaration
        //
        "TYPE → IDENTIFIER" => {
            return vec![
                SyntaxElement("IDENTIFIER".into()),
            ]
        }
        "TYPE → float" => {
            return vec![
                SemanticElement(Box::new(PushType{type_string: "float".to_string()})),
                SyntaxElement("float".into()),
            ]
        }
        "TYPE → integer" => {
            return vec![
                SemanticElement(Box::new(PushType{type_string: "int".to_string()})),
                SyntaxElement("integer".into()),
            ]
        }
        "START → LOCALVARDECL eof" => {
            return vec![
                SyntaxElement("LOCALVARDECL".to_string()),
                SyntaxElement("eof".to_string()),
                SemanticElement(Box::new(CheckStackOneNode)),
            ];
        }
        "LOCALVARDECL → localvar IDENTIFIER colon TYPE ARRAYLIST semi" => {
            return vec![
                SyntaxElement("localvar".to_string()),
                SyntaxElement("IDENTIFIER".to_string()),
                SyntaxElement("colon".to_string()),
                SyntaxElement("TYPE".to_string()),
                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("ARRAYLIST".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "ArraySize".into()})),
                SyntaxElement("semi".to_string()),
                SemanticElement(Box::new(LocalVarGather))
            ];
        }
        "ARRAYSIZPOSTFIX → rsqbr" => {
            return vec![
                SemanticElement(Box::new(PushArraySize{size: 0})),
                SyntaxElement("rsqbr".into()),
            ]
        }
        "ARRAYSIZPOSTFIX → intlit rsqbr" => {
            return vec![
                SemanticElement(Box::new(PushArraySize{size: 0})),
                SyntaxElement("intlit".into()),
                SyntaxElement("rsqbr".into()),
            ]
        }
        "ARRAYLIST → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "ARRAYLIST → ARRAYSIZE ARRAYLIST" => {
            return get_only_syntax_elements(production_string)
        }
        "ARRAYSIZE → lsqbr ARRAYSIZPOSTFIX" => {
            return get_only_syntax_elements(production_string)
        },
        "START → FUNCDEF eof" => {
            return vec![
                SyntaxElement("FUNCDEF".to_string()),
                SyntaxElement("eof".to_string()),
                SemanticElement(Box::new(CheckStackOneNode)),
            ];
        }
        "LISTLOCALVARDECL → LOCALVARDECL LISTLOCALVARDECL" => {
            return get_only_syntax_elements(production_string)
        }
        "LISTLOCALVARDECL → &epsilon" =>  {
            return get_only_syntax_elements(production_string)
        }
        "PROGRAM → LISTFUNCTIONS" => {
            return vec![
                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTFUNCTIONS".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "FunctionList".into()})),
                SemanticElement(Box::new(ProgramGather))
            ];
        },
        "START → PROGRAM eof" => {
            return vec![
                SyntaxElement("PROGRAM".to_string()),
                SyntaxElement("eof".to_string()),
                SemanticElement(Box::new(CheckStackOneNode)),
            ];
        },
        "LISTFUNCTIONS → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "LISTFUNCTIONS → FUNCDEF LISTFUNCTIONS" =>  {
            return get_only_syntax_elements(production_string)
        }
        "LISTASSIGNSTATEMENTS → ASSIGNSTAT LISTASSIGNSTATEMENTS" => {
            return get_only_syntax_elements(production_string)
        }
        "LISTASSIGNSTATEMENTS → &epsilon" =>  {
            return get_only_syntax_elements(production_string)
        }
        "FUNCDEF → function IDENTIFIER lpar rpar lcurbr LISTLOCALVARDECL rcurbr" => {
            return vec![
                SyntaxElement("function".into()),
                SyntaxElement("IDENTIFIER".into()),
                SyntaxElement("lpar".to_string()),
                SyntaxElement("rpar".to_string()),
                SyntaxElement("lcurbr".to_string()),
                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTLOCALVARDECL".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "LocalVarDeclList".into()})),
                SyntaxElement("rcurbr".to_string()),
                SemanticElement(Box::new(FunctionGather))
            ];
        }
        "FUNCDEF → function IDENTIFIER lpar rpar lcurbr LISTLOCALVARDECL LISTASSIGNSTATEMENTS rcurbr" => {
            return vec![
                SyntaxElement("function".into()),
                SyntaxElement("IDENTIFIER".into()),
                SyntaxElement("lpar".to_string()),
                SyntaxElement("rpar".to_string()),
                SyntaxElement("lcurbr".to_string()),

                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTLOCALVARDECL".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "LocalVarDeclList".into()})),

                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTASSIGNSTATEMENTS".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "AssignStatList".into()})),

                SyntaxElement("rcurbr".to_string()),
                SemanticElement(Box::new(FunctionGatherFull))
            ];
        }
        "FUNCDEF → function IDENTIFIER lpar FPARAMS rpar lcurbr LISTLOCALVARDECL LISTASSIGNSTATEMENTS rcurbr" => {
            return vec![
                SemanticElement(Box::new(MarkListBegin)),

                SyntaxElement("function".into()),
                SyntaxElement("IDENTIFIER".into()),

                SyntaxElement("lpar".to_string()),

                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("FPARAMS".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "FuncParamsList".into()})),

                SyntaxElement("rpar".to_string()),

                SyntaxElement("lcurbr".to_string()),

                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTLOCALVARDECL".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "LocalVarDeclList".into()})),

                SemanticElement(Box::new(MarkListBegin)),
                SyntaxElement("LISTASSIGNSTATEMENTS".to_string()),
                SemanticElement(Box::new(CollectList{list_name: "AssignStatList".into()})),

                SyntaxElement("rcurbr".to_string()),

                SemanticElement(Box::new(CollectList{list_name: "Function".into()})),
            ];
        }
        "ASSIGNSTAT → IDENTIFIER equal ASSIGNEDVALUE semi" => {
            return vec![
                SyntaxElement("IDENTIFIER".into()),
                SyntaxElement("equal".to_string()),
                SyntaxElement("ASSIGNEDVALUE".to_string()),
                SyntaxElement("semi".to_string()),
                SemanticElement(Box::new(AssignStatementGather))
            ];
        }
        "ASSIGNEDVALUE → IDENTIFIER" => {
            return vec![
                SyntaxElement("IDENTIFIER".into()),
            ];
        }
        "ASSIGNEDVALUE → ARITHEXPR" =>  {
            return get_only_syntax_elements(production_string)
        }
        // Function parameters
        "FPARAMS → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "FPARAMS → IDENTIFIER colon TYPE ARRAYLIST FPARAMSTAIL" => {
            return vec![
                SemanticElement(Box::new(MarkListBegin)),

                SyntaxElement("IDENTIFIER".into()),
                SyntaxElement("colon".into()),
                SyntaxElement("TYPE".into()),
                SyntaxElement("ARRAYLIST".into()),

                SemanticElement(Box::new(CollectList{list_name: "FunctionParam".into()})),

                SyntaxElement("FPARAMSTAIL".into()),
            ];
        }
        "FPARAMSTAIL → &epsilon" => {
            return get_only_syntax_elements(production_string)
        }
        "FPARAMSTAIL → comma FPARAMS" => {
            return get_only_syntax_elements(production_string)
        }
        "IDENTIFIER → id" => {
            return vec![
                SemanticElement(Box::new(PushIdentifier)),
                SyntaxElement("id".into()),
            ];
        }
        _ => panic!("Can't add semantics to ({}) at the moment", production_string.as_str())
    };
}
