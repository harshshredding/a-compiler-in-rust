---
colorlinks: true
geometry: margin=2cm
---


# Assignment 3

### Attribute Grammar
The attribute grammar is in `./attribute_grammar.grm`. Please read the design section below for definitions of 
used semantic actions.

### Design
Because I am using **table driven** parsing approach, I introduced special Semantic Symbols on the right hand side of productions(see below for list). 
Following are the types of semantic actions used:

- `@Push`: Pushes the given value on the semantic stack, and a corresponding node is created in the graph.
- `@Gather`: Gather/Pop values from the top of the semantic stack and creates a node that corresponds to the gathered values.
The gathered value is then pushed on the stack.
- `@MarkBeginList`: Pushes a marker representing the beginning of a list on semantic stack.
- `@CollectList`: Collects everything until the last list marker from the stack and creates a new node representing the collected items.
- `@CheckStackOneNode`: Ensures that the stack only has one node on it. This check is helpful when we have generated the entire program.

Each of these semantic actions trigger a piece of code which modifies the semantic stack or creates graph nodes.


I grew the semantic grammar organically (starting from a small grammar subset and then adding more complexity) while testing it.
Currently, the grammar can accurately represent:
- Arithmetic Expressions
- Function Implementations
- Assignment Statements
- Local Variable Declarations 

Please see `src/test_out` directory for several dot file outputs and corresponding svf files.

### Use Of Tools
- **University Of Calgary Website**:  For generations of LL(1) parsing tables for my table driven parser.
- **GraphViz**: Visualizing my trees.
- **Rust Lang**: The tokenizer, syntax analyzer, and semantic analyzer has been implemented in Rust.

## AST Output for test cases
Test source files are in `src/syntax_tests/src` and the corresponding ASTs(`dot` files and `svg` files) are in `src/test_out`.

## Driver
Place any source you want to test in `src/syntax_tests/src/custom_test.src`.

Then run `cargo test` in terminal. The AST output and the derivation will be in `src/test_out/custom_test.dot` 
and `src/syntax_tests/out/custom_test.derivation` respectively.
