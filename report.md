---
colorlinks: true
geometry: margin=2cm
---


# Assignment 3

### Attribute Grammar
The attribute grammar is in `./attribute_grammar.grm`.

Following are the types of semantic actions used:

- @Push: Pushes the given value on the semantic stack, and a corresponding node is created in the graph.
- @Gather: Gather/Pop values from the top of the semantic stack and creates a node that corresponds to the gathered values.
The gathered value is then pushed on the stack.
- @MarkBeginList: Pushes a marker representing the beginning of a list on semantic stack.
- @CollectList: Collects everything until the last list marker from the stack and creates a new node representing the collected items.
- @CheckStackOneNode: Ensures that the stack only has one node on it. This check is helpful when we have generated the entire program.