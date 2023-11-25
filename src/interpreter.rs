use std::collections::HashMap;
// #![allow(warnings)]

// Define a struct for storing variables and their values
#[derive(Clone,Debug)]
pub struct Variable {
    name: String,
    value: Value,
}

// Define a Value enum for storing different types of variable values
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    None,
    // scalar
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    // compund
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Set(Vec<Value>),
    // Dictionary(HashMap<String, Value>),
}

// Define a struct for storing functions and their parameters and code blocks
#[derive(Clone,Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Option<Value>)>,
    pub body: Vec<Statement>,
}

// Define a Statement enum for storing different types of statements
#[derive(Clone,Debug)]
pub enum Statement {
    Assignment(String, Expression),
    Expression(Expression),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    While(Expression, Vec<Statement>),
    For(String, Expression, Expression, Vec<Statement>),
    Function(Function),
    Return(Expression),
    Import(String),
}

#[derive(Clone, Debug)]
// Define an Expression enum for storing different types of expressions
pub enum Expression {
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Literal(Value),
    Variable(String),
    FunctionCall(String, Vec<Expression>),
}

// Define a BinaryOperator enum for storing different types of binary operators
#[derive(Clone,Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

// Define a UnaryOperator enum for storing different types of unary operators
#[derive(Clone,Debug)]
pub enum UnaryOperator {
    Not,
    Minus,
}

// Define a struct for storing the interpreter state
#[derive(Clone,Debug)]
struct Interpreter {
    variables: Vec<Variable>,
    functions: Vec<Function>,
}

// Implement the Interpreter struct
impl Interpreter {
    // Define a function to evaluate an expression and return its value
    fn eval_expression(&self, expr: &Expression) -> Value {
        match expr {
            Expression::Binary(left, op, right) => {
                let left_value = self.eval_expression(left);
                let right_value = self.eval_expression(right);
                match op {
                    BinaryOperator::Add => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
                        (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
                        (Value::String(l), Value::String(r)) => Value::String(format!("{}{}", l, r)),
                        _ => panic!("Invalid operands for addition"),
                    },
                    BinaryOperator::Subtract => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
                        (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
                        _ => panic!("Invalid operands for subtraction"),
                    },
                    BinaryOperator::Multiply => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
                        (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
                        _ => panic!("Invalid operands for multiplication"),
                    },
                    BinaryOperator::Divide => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l / r),
                        (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
                        _ => panic!("Invalid operands for division"),
                    },
                    BinaryOperator::Modulo => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
                        _ => panic!("Invalid operands for modulo"),
                    },
                    BinaryOperator::Equal => Value::Boolean(left_value == right_value),
                    BinaryOperator::NotEqual => Value::Boolean(left_value != right_value),
                    BinaryOperator::LessThan => Value::Boolean(left_value < right_value),
                    BinaryOperator::GreaterThan => Value::Boolean(left_value > right_value),
                    BinaryOperator::LessThanOrEqual => Value::Boolean(left_value <= right_value),
                    BinaryOperator::GreaterThanOrEqual => Value::Boolean(left_value >= right_value),
                    BinaryOperator::And => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l & r),
                        _ => panic!("Invalid operands for bitwise AND"),
                    },
                    BinaryOperator::Or => match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Value::Integer(l | r),
                        _ => panic!("Invalid operands for bitwise OR"),
                    },
                    // Implement other binary operators as needed
                    _ => unimplemented!(),
                }
            }
            Expression::Unary(op, expr) => {
                let value = self.eval_expression(expr);
                match op {
                    UnaryOperator::Not => match value {
                        Value::Boolean(b) => Value::Boolean(!b),
                        _ => panic!("Invalid operand for 'not' operator"),
                    },
                    UnaryOperator::Minus => match value {
                        Value::Integer(i) => Value::Integer(-i),
                        Value::Float(f) => Value::Float(-f),
                        _ => panic!("Invalid operand for minus operator"),
                    },
                }
            }
            Expression::Literal(value) => value.clone(),
            Expression::Variable(name) => {
                let variable = self
                    .variables
                    .iter()
                    .find(|v| v.name == *name)
                    .expect("Variable not found");
                variable.value.clone()
            }
            Expression::FunctionCall(name, args) => {
                let function = self
                    .functions
                    .iter()
                    .find(|f| f.name == *name)
                    .expect("Function not declared");
                let mut local_variables = Vec::new();
                for (param, arg) in function.params.iter().zip(args.iter()) {
                    dbg!(&param);
                    dbg!(&arg);
                    let (param, default) = param;
                    dbg!(&default);
                    local_variables.push(Variable {
                        name: param.clone(),
                        // value: if value { value } else {default.unwrap()},
                        value: self.eval_expression(arg),
                    });
                }
                let mut interpreter = Interpreter {
                    variables: local_variables,
                    functions: self.functions.clone(),
                };
                for statement in &function.body {
                    interpreter.eval_statement(statement);
                }
                // Return value is not handled in this example
                Value::None
            }
        }
    }

    // Define a function to evaluate a statement
    fn eval_statement(&mut self, statement: &Statement) {
        dbg!(&statement);
        match statement {
            Statement::Import(name) => {}
            Statement::Assignment(name, expr) => {
                let value = self.eval_expression(expr);
                if let Some(variable) = self.variables.iter_mut().find(|v| v.name == *name) {
                    variable.value = value;
                }
                else {
                    self.variables.push(Variable { name: name.clone(), value });
                }
            }
            Statement::Expression(expr) => {
                let ret = self.eval_expression(expr);
                dbg!(ret);
            }
            Statement::If(cond, if_block, else_block) => {
                let cond_value = self.eval_expression(cond);
                if let Value::Boolean(true) = cond_value {
                    for statement in if_block {
                        self.eval_statement(statement);
                    }
                } else if let Some(block) = else_block {
                    for statement in block {
                        self.eval_statement(statement);
                    }
                }
            }
            Statement::While(cond, block) => {
                while let Value::Boolean(true) = self.eval_expression(cond) {
                    for statement in block {
                        self.eval_statement(statement);
                    }
                }
            }
            Statement::For(var, start, end, block) => {
                let start_value = self.eval_expression(start);
                let end_value = self.eval_expression(end);
                if let (Value::Integer(start), Value::Integer(end)) = (start_value, end_value) {
                    for i in start..=end {
                        self.variables.push(Variable {
                            name: var.clone(),
                            value: Value::Integer(i),
                        });
                        for statement in block {
                            self.eval_statement(statement);
                        }
                        self.variables.retain(|v| v.name != *var);
                    }
                } else {
                    panic!("Invalid range values for 'for' loop");
                }
            }
            Statement::Function(function) => {
                self.functions.push(function.clone());
            }
            Statement::Return(expr) => {
                let _ = self.eval_expression(expr);
                // Handling return value is not shown in this example
            }
        }
    }
}


pub fn main() {
    // Create an instance of the interpreter
    let mut interpreter = Interpreter {
        variables: Vec::new(),
        functions: Vec::new(),
    };

    // dbg!(Expression::Literal(Value::Integer(10)));
    
    let function = Function {
        name: String::from("add"),
        params: vec![(String::from("a"), None), (String::from("b"),None)],
        body: vec![
            Statement::Expression(Expression::Variable(String::from("a"))),
            Statement::Expression(Expression::Variable(String::from("b"))),
            Statement::Return(Expression::Literal(Value::None))
        ],
    };

    // Example code
    let statements = vec![
        Statement::Function(function),
        // Statement::If(
        //     Expression::Binary(
        //         Box::new(Expression::Literal(Value::Integer(10))),
        //         BinaryOperator::NotEqual,
        //         Box::new(Expression::Literal(Value::Integer(20))),
        //     ),
        //     vec![Statement::Assignment(String::from("x"),Expression::Literal(Value::Float(3.6)))],
        //     Some(vec![Statement::Assignment(String::from("x"),Expression::Literal(Value::Float(33.6)))]),
        // ),
        // Statement::Assignment(
        //     String::from("u"),
        //     Expression::Binary(
        //         Box::new(Expression::Literal(Value::Integer(17))),
        //         BinaryOperator::NotEqual,
        //         Box::new(Expression::Literal(Value::Integer(2))),
        //     ),
        // ),
        // Statement::Expression(Expression::FunctionCall("add".to_string(),vec![Expression::Literal(Value::Float(4.6)),Expression::Literal(Value::Float(43.6))])),
        Statement::Expression(Expression::Literal(Value::Float(43.6))),
        // Statement::Expression(Expression::Literal(Value::String("x".to_string()))),
    ];

    // Evaluate the statements
    for statement in statements {
        let s = interpreter.eval_statement(&statement);
        dbg!(s);
    }
}
