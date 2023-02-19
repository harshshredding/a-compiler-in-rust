<START>              ::= {{<classDeclOrFuncDef>}}
<classDeclOrFuncDef> ::= <classDecl>
                      |  <funcDef> 
<classDecl>          ::= 'class' 'id' [['isa' 'id' {{',' 'id'}}]] '{' {{<visibility> <memberDecl>}} '}' ';'
<visibility>         ::= 'public' | 'private' | EPSILON
<memberDecl>         ::= <memberFuncDecl> 
                      |  <memberVarDecl>  
<memberFuncDecl>     ::= 'function' 'id' ':' '(' <fParams> ')' 'arrow' <returnType> ';' 
                      |  'constructor' ':' '(' <fParams> ')' ';' 
<memberVarDecl>      ::= 'attribute' 'id' ':' <type> {{<arraySize>}} ';'
<funcDef>            ::= <funcHead> <funcBody> 
<funcHead>           ::= 'function' [['id' 'sr']] 'id' '(' <fParams> ')' 'arrow' <returnType> 
                      |  'function' 'id' 'sr' 'constructor' '(' <fParams> ')'
<funcBody>           ::= '{' {{<localVarDeclOrStmt>}} '}'
<localVarDeclOrStmt> ::= <localVarDecl>
                      |  <statement>
<localVarDecl>       ::= 'localVar' 'id' ':' <type> {{<arraySize>}} ';'
                      |  'localVar' 'id' ':' <type> '(' <aParams> ')' ';'
<statement>          ::= <assignStat> ';'
                      |  'if'     '(' <relExpr> ')' 'then' <statBlock> 'else' <statBlock> ';'
                      |  'while'  '(' <relExpr> ')' <statBlock> ';'
                      |  'read'   '(' <variable> ')' ';'
                      |  'write'  '(' <expr> ')' ';'
                      |  'return' '(' <expr> ')' ';'
                      |  <functionCall> ';'
<assignStat>         ::= <variable> <assignOp> <expr>
<statBlock>          ::= '{' {{<statement>}} '}' | <statement> | EPSILON  
<expr>               ::= <arithExpr> | <relExpr>
<relExpr>            ::= <arithExpr> <relOp> <arithExpr>
<arithExpr>          ::= <arithExpr> <addOp> <term> | <term> 
<sign>               ::= '+' | '-'
<term>               ::= <term> <multOp> <factor> | <factor>
<factor>             ::= <variable>
                      |  <functionCall>
                      |  'intLit' | 'floatLit'
                      |  '(' <arithExpr> ')'
                      |  'not' <factor>
                      |  <sign> <factor>
<variable>           ::= {{<idnest>}} 'id' {{<indice>}}
<functionCall>       ::= {{<idnest>}} 'id' '(' <aParams> ')'
<idnest>             ::= 'id' {{<indice>}} '.'
                      |  'id' '(' <aParams> ')' '.'
<indice>             ::= '[' <arithExpr> ']'
<arraySize>          ::= '[' 'intLit' ']' | '[' ']'
<type>               ::= 'integer' | 'float' | 'id'
<returnType>         ::= <type>
                      |  'void'
<fParams>            ::= 'id' ':' <type> {{<arraySize>}} {{<fParamsTail>}} | EPSILON  
<aParams>            ::= <expr> {{<aParamsTail>}} | EPSILON 
<fParamsTail>        ::= ',' 'id' ':' <type> {{<arraySize>}}
<aParamsTail>        ::= ',' <expr>
<assignOp>           ::= '='
<relOp>              ::= 'eq' | 'neq' | 'lt' | 'gt' | 'leq' | 'geq' 
<addOp>              ::= '+' | '-' | 'or' 
<multOp>             ::= '*' | '/' | 'and'
