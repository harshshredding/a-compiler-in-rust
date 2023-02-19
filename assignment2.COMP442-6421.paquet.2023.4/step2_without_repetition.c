<START> ::= <rept-START0> 

<aParams> ::= <expr> <rept-aParams1> 
<aParams> ::= EPSILON 

<aParamsTail> ::= ',' <expr> 

<addOp> ::= '+' 
<addOp> ::= '-' 
<addOp> ::= 'or' 

<arithExpr> ::= <arithExpr> <addOp> <term> 
<arithExpr> ::= <term> 

<arraySize> ::= '[' 'intLit' ']' 
<arraySize> ::= '[' ']' 

<assignOp> ::= '=' 

<assignStat> ::= <variable> <assignOp> <expr> 

<classDecl> ::= 'class' 'id' <opt-class-inheritance> '{' <rept-classDecl4> '}' ';' 

<classDeclOrFuncDef> ::= <classDecl> 
<classDeclOrFuncDef> ::= <funcDef> 

<expr> ::= <arithExpr> 
<expr> ::= <relExpr> 

<fParams> ::= 'id' ':' <type> <rept-fParams3> <rept-fParams4> 
<fParams> ::= EPSILON 

<fParamsTail> ::= ',' 'id' ':' <type> <rept-fParamsTail4> 

<factor> ::= <variable> 
<factor> ::= <functionCall> 
<factor> ::= 'intLit' 
<factor> ::= 'floatLit' 
<factor> ::= '(' <arithExpr> ')' 
<factor> ::= 'not' <factor> 
<factor> ::= <sign> <factor> 

<funcBody> ::= '{' <rept-funcBody1> '}' 

<funcDef> ::= <funcHead> <funcBody> 

<funcHead> ::= 'function' <opt-function-head-class-member> 'id' '(' <fParams> ')' 'arrow' <returnType> 
<funcHead> ::= 'function' 'id' 'sr' 'constructor' '(' <fParams> ')' 

<functionCall> ::= <rept-functionCall0> 'id' '(' <aParams> ')' 

<idnest> ::= 'id' <rept-idnest1> '.' 
<idnest> ::= 'id' '(' <aParams> ')' '.' 

<indice> ::= '[' <arithExpr> ']' 

<localVarDecl> ::= 'localVar' 'id' ':' <type> <rept-localVarDecl4> ';' 
<localVarDecl> ::= 'localVar' 'id' ':' <type> '(' <aParams> ')' ';' 

<localVarDeclOrStmt> ::= <localVarDecl> 
<localVarDeclOrStmt> ::= <statement> 

<memberDecl> ::= <memberFuncDecl> 
<memberDecl> ::= <memberVarDecl> 

<memberFuncDecl> ::= 'function' 'id' ':' '(' <fParams> ')' 'arrow' <returnType> ';' 
<memberFuncDecl> ::= 'constructor' ':' '(' <fParams> ')' ';' 

<memberVarDecl> ::= 'attribute' 'id' ':' <type> <rept-memberVarDecl4> ';' 

<multOp> ::= '*' 
<multOp> ::= '/' 
<multOp> ::= 'and' 

<opt-class-inheritance> ::= 'isa' 'id' <rept-opt-class-inheritance2> 
<opt-class-inheritance> ::= EPSILON 

<opt-function-head-class-member> ::= 'id' 'sr' 
<opt-function-head-class-member> ::= EPSILON 

<relExpr> ::= <arithExpr> <relOp> <arithExpr> 

<relOp> ::= 'eq' 
<relOp> ::= 'neq' 
<relOp> ::= 'lt' 
<relOp> ::= 'gt' 
<relOp> ::= 'leq' 
<relOp> ::= 'geq' 

<rept-START0> ::= <classDeclOrFuncDef> <rept-START0> 
<rept-START0> ::= EPSILON 

<rept-aParams1> ::= <aParamsTail> <rept-aParams1> 
<rept-aParams1> ::= EPSILON 

<rept-classDecl4> ::= <visibility> <memberDecl> <rept-classDecl4> 
<rept-classDecl4> ::= EPSILON 

<rept-fParams3> ::= <arraySize> <rept-fParams3> 
<rept-fParams3> ::= EPSILON 

<rept-fParams4> ::= <fParamsTail> <rept-fParams4> 
<rept-fParams4> ::= EPSILON 

<rept-fParamsTail4> ::= <arraySize> <rept-fParamsTail4> 
<rept-fParamsTail4> ::= EPSILON 

<rept-funcBody1> ::= <localVarDeclOrStmt> <rept-funcBody1> 
<rept-funcBody1> ::= EPSILON 

<rept-functionCall0> ::= <idnest> <rept-functionCall0> 
<rept-functionCall0> ::= EPSILON 

<rept-idnest1> ::= <indice> <rept-idnest1> 
<rept-idnest1> ::= EPSILON 

<rept-localVarDecl4> ::= <arraySize> <rept-localVarDecl4> 
<rept-localVarDecl4> ::= EPSILON 

<rept-memberVarDecl4> ::= <arraySize> <rept-memberVarDecl4> 
<rept-memberVarDecl4> ::= EPSILON 

<rept-opt-class-inheritance2> ::= ',' 'id' <rept-opt-class-inheritance2> 
<rept-opt-class-inheritance2> ::= EPSILON 

<rept-statBlock1> ::= <statement> <rept-statBlock1> 
<rept-statBlock1> ::= EPSILON 

<rept-variable0> ::= <idnest> <rept-variable0> 
<rept-variable0> ::= EPSILON 

<rept-variable2> ::= <indice> <rept-variable2> 
<rept-variable2> ::= EPSILON 

<returnType> ::= <type> 
<returnType> ::= 'void' 

<sign> ::= '+' 
<sign> ::= '-' 

<statBlock> ::= '{' <rept-statBlock1> '}' 
<statBlock> ::= <statement> 
<statBlock> ::= EPSILON 

<statement> ::= <assignStat> ';' 
<statement> ::= 'if' '(' <relExpr> ')' 'then' <statBlock> 'else' <statBlock> ';' 
<statement> ::= 'while' '(' <relExpr> ')' <statBlock> ';' 
<statement> ::= 'read' '(' <variable> ')' ';' 
<statement> ::= 'write' '(' <expr> ')' ';' 
<statement> ::= 'return' '(' <expr> ')' ';' 
<statement> ::= <functionCall> ';' 

<term> ::= <term> <multOp> <factor> 
<term> ::= <factor> 

<type> ::= 'integer' 
<type> ::= 'float' 
<type> ::= 'id' 

<variable> ::= <rept-variable0> 'id' <rept-variable2> 

<visibility> ::= 'public' 
<visibility> ::= 'private' 
<visibility> ::= EPSILON 


