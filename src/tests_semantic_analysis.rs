use super::semantic_analysis::*;

#[cfg(test)]

#[test]
fn test_semantic_action() {
    let production = Production::SemanticProduction(CreatePlusNode);
    let mut stack : Vec<String> = vec![];
    match production {
        Production::SemanticProduction(handler) =>  {
            handler.take_action(&mut stack);
        },
        _ => {
            panic!("Wrong production type!");
        }
    }
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().unwrap(), "+");
}

