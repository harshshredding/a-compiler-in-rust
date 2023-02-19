<START> ::= {{ <classDeclOrFuncDef> }} 

<aParams> ::= <expr> {{ <aParamsTail> }} 
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

<classDecl> ::= 'class' 'id' <opt-class-inheritance> '{' {{ <visibility> <memberDecl> }} '}' ';' 

<classDeclOrFuncDef> ::= <classDecl> 
<classDeclOrFuncDef> ::= <funcDef> 

<expr> ::= <arithExpr> 
<expr> ::= <relExpr> 

<fParams> ::= 'id' ':' <type> {{ <arraySize> }} {{ <fParamsTail> }} 
<fParams> ::= EPSILON 

<fParamsTail> ::= ',' 'id' ':' <type> {{ <arraySize> }} 

<factor> ::= <variable> 
<factor> ::= <functionCall> 
<factor> ::= 'intLit' 
<factor> ::= 'floatLit' 
<factor> ::= '(' <arithExpr> ')' 
<factor> ::= 'not' <factor> 
<factor> ::= <sign> <factor> 

<funcBody> ::= '{' {{ <localVarDeclOrStmt> }} '}' 

<funcDef> ::= <funcHead> <funcBody> 

<funcHead> ::= 'function' <opt-function-head-class-member> 'id' '(' <fParams> ')' 'arrow' <returnType> 
<funcHead> ::= 'function' 'id' 'sr' 'constructor' '(' <fParams> ')' 

<functionCall> ::= {{ <idnest> }} 'id' '(' <aParams> ')' 

<idnest> ::= 'id' {{ <indice> }} '.' 
<idnest> ::= 'id' '(' <aParams> ')' '.' 

<indice> ::= '[' <arithExpr> ']' 

<localVarDecl> ::= 'localVar' 'id' ':' <type> {{ <arraySize> }} ';' 
<localVarDecl> ::= 'localVar' 'id' ':' <type> '(' <aParams> ')' ';' 

<localVarDeclOrStmt> ::= <localVarDecl> 
<localVarDeclOrStmt> ::= <statement> 

<memberDecl> ::= <memberFuncDecl> 
<memberDecl> ::= <memberVarDecl> 

<memberFuncDecl> ::= 'function' 'id' ':' '(' <fParams> ')' 'arrow' <returnType> ';' 
<memberFuncDecl> ::= 'constructor' ':' '(' <fParams> ')' ';' 

<memberVarDecl> ::= 'attribute' 'id' ':' <type> {{ <arraySize> }} ';' 

<multOp> ::= '*' 
<multOp> ::= '/' 
<multOp> ::= 'and' 

<opt-class-inheritance> ::= 'isa' 'id' {{ ',' 'id' }} 
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

<returnType> ::= <type> 
<returnType> ::= 'void' 

<sign> ::= '+' 
<sign> ::= '-' 

<statBlock> ::= '{' {{ <statement> }} '}' 
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

<variable> ::= {{ <idnest> }} 'id' {{ <indice> }} 

<visibility> ::= 'public' 
<visibility> ::= 'private' 
<visibility> ::= EPSILON 


