use crate::interpreter::{Value, Expression, Statement, BinaryOperator, UnaryOperator, Function};
use crate::tokenizer::{Token, ParseError, Symbol, Keyword};

impl Token {
    pub fn as_value(&self) -> Option<Value> {
        match self {
            Token::FString(text) => { Some(Value::String(text.clone())) }
            Token::Text(text) => {
                if let Ok(integer) = text.parse::<i64>() {
                    Some(Value::Integer(integer))
                } else if let Ok(float) = text.parse::<f64>() {
                    Some(Value::Float(float))
                } else if let Ok(boolean) = text.parse::<bool>() {
                    Some(Value::Boolean(boolean))
                } else {
                    Some(Value::String(text.clone()))
                }
            }
            _ => None,
        }
    }
}

// Design decisions: - Strings are consecutive chars and if we have symbols and chars into a String we 
// can ignore then to build a single string, but the parser be wraped to this design decision. Using tokenizer 
// as raw interpreter and so build a string from tokens validated gives more abstraction and flexibility 
// to build the AST to syntax.

#[derive(Debug)]
enum SyntaxState{
    IntoStatement,
    IntoParenthesis, //into function call, assigment or expression
    PrevIdentifier, // into assigment line, can destructure
    IntoFunction, //inside scope function
    IntoContainer, //list, tuple, ..
    IntoAssigment, 
    IntoImport,
    // IntoIf, 
    // IntoFor, 
    // IntoWhile, 
    // IntoExpression, 
    EndStatement,
    End,
}

#[derive(Debug)]
enum FunctionState{
    Begin,
    IntoArguments, //list, tuple, ..
}

#[derive(Debug)]
pub struct Parser {
    // using drain to remove tokens allow functions to get all tokens and jump
    current_ident: i32,
    current_token: usize,
    tokens: Vec<Token>,
    ast: Vec<Statement>,
    state: SyntaxState,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current_ident: 0,
            current_token: 0,
            tokens: tokens,
            ast: Vec::new(),
            state:SyntaxState::IntoStatement,
        }
    }

    fn parse_token(&mut self) -> Result<(), ParseError> {

        // dbg!(&self);
        let token = self.tokens.get(self.current_token).cloned().unwrap();
        dbg!(&self.state);
        dbg!(&token);
        self.current_token +=1;
        
        match self.state {
            SyntaxState::IntoImport => {
                while let Some(token) = self.tokens.get(self.current_token) {
                    if let Token::LineBreak = token {
                        break;
                    }
                    match token {
                        Token::Identifier(ident) => {
                            dbg!(&ident);
                            self.ast.push(Statement::Import(ident.to_string()))
                        }
                        // ignore if not identifier, comma are not identifier too
                        _ => {}
                    }
                    self.current_token +=1;
                }
                self.state = SyntaxState::IntoStatement;
                self.parse_token();
            } 
            SyntaxState::IntoStatement => {
                match &token {
                    Token::Keyword(keyword) => {
                        match keyword {
                            Keyword::Import => { 
                                self.state = SyntaxState::IntoImport;
                                self.parse_token();
                            }
                            Keyword::Def => {
                                dbg!(keyword);
                                self.state = SyntaxState::IntoFunction;
                                self.parse_token();
                            }
                            Keyword::Return => {
                                // let expr = Expression::Number(0); // placeholder expression
                                // self.ast.push(AST::Expression(expr));
                                self.state = SyntaxState::End;
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken(token));
                            }
                        }
                    },

                    Token::Space | Token::LineBreak => {
                        self.parse_token();
                    }

                    Token::Identifier(ident) => {
                        // dbg!(ident);
                        self.state = SyntaxState::PrevIdentifier;
                        // self.temp.push(token);
                        // TODO: handle identifier
                    },
                    // Token::Symbol::LeftParen => {
                    //     // TODO: handle function call / expression
                    // },
                    // Token::Symbol::LeftBracket | Token::Symbol::LeftParen | Token::Symbol::LeftCurlyBrace => {
                    //     self.state = SyntaxState::IntoContainer;
                    //     // TODO: handle container types (list, tuple, dict)
                    // },
                    _ => return Err(ParseError::UnexpectedToken(token)),
                }
            },
            SyntaxState::IntoParenthesis => {
                match token {
                    Token::Identifier(name) => { },
                    Token::Space => {
                        self.parse_token();
                    }
                    Token::Symbol(symbol) => {
                        match symbol {
                            // assign default values into functions
                            // destructure assigment if not in a function scope
                            Symbol::Assign => {}
                            // separate values, identifiers, literals, etc..
                            Symbol::Comma => {}
                            // ignore
                            // string literal
                            Symbol::DoubleQuote | Symbol::DoubleQuote => {}
                            _ => todo!()
                        }
                    }
                    _ => { return Err(ParseError::UnexpectedToken(token)) },
                }
            },
            SyntaxState::PrevIdentifier => {
                match token {
                    Token::Space => {
                        self.parse_token();
                    }
                    Token::Symbol(s) => {
                        match s{
                            Symbol::Assign => {
                                self.state = SyntaxState::IntoAssigment;
                            }
                            _ => {} 
                        }
                        // let targets = vec![]; // placeholder for assignment target(s)
                        // self.ast.push(AST::Assignment { targets, value: None });
                    },
                    _ => return Err(ParseError::UnexpectedToken(token)),
                }
            },
            SyntaxState::IntoFunction => {
                match token {
                    // ignore
                    Token::Space => {
                        self.parse_token();
                    }
                    // add to ast the signature of the function
                    Token::Identifier(function_identifier) => {
                        let mut fn_state = FunctionState::Begin;
                        let mut fn_params : Vec<(String, Option<Value>)> = Vec::new();
                        // let mut fn_statements : Vec<Statement> = Vec::new();

                        while let Some(token) = self.tokens.get(self.current_token) {
                            if let Token::Symbol(Symbol::Colon) = token {
                                break;
                            }
                            match self.tokens.get(self.current_token).cloned().unwrap(){
                                Token::Symbol(s) => {
                                    match s {
                                        Symbol::LeftParen => {
                                            fn_state = FunctionState::IntoArguments;
                                        }
                                        // set default arguments
                                        Symbol::Assign => {
                                            self.current_token +=1;
                                            let v = self.tokens.get(self.current_token).cloned().unwrap();
                                            match &v {
                                                Token::Text(t)|Token::FString(t) => {
                                                    fn_params.last_mut().map(|params| params.1 = Some(v.as_value().unwrap()));
                                                    dbg!(&fn_params);
                                                }
                                                _ => { return Err(ParseError::UnexpectedToken(v)); }
                                            }
                                        }
                                        _ => { }
                                    }
                                }
                                Token::Identifier(iden) => {
                                    match fn_state {
                                        FunctionState::IntoArguments => {
                                            fn_params.push((iden, None));
                                        }
                                        _ => { }
                                    }
                                 }
                                _ => { }
                            }
                            dbg!(&token);
                            self.current_token +=1;
                        }
                        // generate a function scope into ast
                        let function = Function {
                            name: function_identifier,
                            params: fn_params,
                            body: Vec::new(),
                        };
                        self.ast.push(Statement::Function(function));
                        self.state = SyntaxState::IntoStatement;
                        self.current_token +=1;
                        self.parse_token();
                        
                    }
                    _ => return Err(ParseError::UnexpectedToken(token)),
                }
            },
            SyntaxState::IntoContainer => {
                // TODO: handle container values (list items, tuple items, dict entries)
            },
            SyntaxState::EndStatement => {
                // not means end of program, but end of each statement

                // add statement to a function scope if exist else add statement direct into global scope
                let function_statements = if let Some(Statement::Function(function)) = self.ast.last_mut() {
                    // Get the mutable reference to the Function statements Vec
                    function 
                } else {
                    panic!("Expected Function statement in the ast vector");
                };
                function_statements.body.push(Statement::Expression(Expression::Literal(Value::Integer(6))));
                dbg!(self.ast.last());
            }
            SyntaxState::End => {
                return Ok(())
                // return Err(ParseError::UnexpectedToken(token));
            }
            SyntaxState::IntoAssigment => {
                match token {
                    Token::Space => {}
                    _ => {
                        dbg!(&token);
                        // let _id = match self.temp.pop() {
                        //     Some(name) => match name {
                        //         // Token::Identifier(n) => { 
                        //         //     dbg!("------------------");
                        //         //     dbg!(&n);
                        //         //     n }
                        //         _ => { "No name".to_string() }
                        //     }
                        //     _ => { "No name".to_string() }
                        //     // _ => return Err(ParseError::UnexpectedToken(token)),
                        // };
                        // let _st = Statement::Assignment(_id,Expression::Literal(Value::Integer(3)));
                        // self.ast.push(_st);
                        self.state = SyntaxState::End;
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), ParseError> {
        // Implement expression parsing logic here
        Ok(())
    }

    fn parse_block(&mut self) -> Result<(), ParseError> {
        // Implement block parsing logic here
        Ok(())
    }

    pub fn parse_tokens(&mut self) -> Result<Vec<Statement>, ParseError> {
        // dbg!(&tokens);
        self.parse_token()?;
        Ok(self.ast.clone())
    }

}

// IntoStatement