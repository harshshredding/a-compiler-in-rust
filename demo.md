## Removed following ambiguities:
The grammar is not LL(1) because:

- ARRAYSIZE has a first set conflict.
- EXPR has a first set conflict.
- FACTOR has a first set conflict.
- FUNCHEAD has a first set conflict.
- IDNEST has a first set conflict.
- LOCALVARDECL has a first set conflict.
- OPTFUNCTIONHEADCLASSMEMBER is nullable with clashing first and follow sets.
- REPTFUNCTIONCALL0 is nullable with clashing first and follow sets.
- REPTVARIABLE0 is nullable with clashing first and follow sets.
- STATEMENT has a first set conflict.

### Simplification :

Statement and Factor:
- `id.id.id.id()`
- `id.id.id.id[]`

instead of
- `id[].id().id[].id()`
- `id.id[].id().id.id[]`
## Table driven approach
- Calgary  
- Generates derivation, but the rules at each step should also be displayed on the side.

## Derivation Output

## Incomplete error output
Currently, the derivation simply halts when an unexpected token is encountered.

## 