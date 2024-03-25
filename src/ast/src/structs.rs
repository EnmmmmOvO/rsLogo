use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BOOLEAN(bool, usize, usize),
    FLOAT(f32, usize, usize),
    VAR(String, usize, usize),
    ADD(Box<Expr>, Box<Expr>, usize, usize),
    SUB(Box<Expr>, Box<Expr>, usize, usize),
    MUL(Box<Expr>, Box<Expr>, usize, usize),
    DIV(Box<Expr>, Box<Expr>, usize, usize),
    EQ(Box<Expr>, Box<Expr>, usize, usize),
    NE(Box<Expr>, Box<Expr>, usize, usize),
    LT(Box<Expr>, Box<Expr>, usize, usize),
    GT(Box<Expr>, Box<Expr>, usize, usize),
    AND(Box<Expr>, Box<Expr>, usize, usize),
    OR(Box<Expr>, Box<Expr>, usize, usize),
    XCOR(usize, usize),
    YCOR(usize, usize),
    HEADING(usize, usize),
    COLOR(usize, usize),
    ERROR(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    IF(Box<Expr>, Vec<Stmt>, usize),
    WHILE(Box<Expr>, Vec<Stmt>, usize),
    MAKE(Box<Assign>, Box<Expr>, usize),
    PENUP(usize),
    PENDOWN(usize),
    FORWARD(Box<Expr>, usize),
    BACK(Box<Expr>, usize),
    LEFT(Box<Expr>, usize),
    RIGHT(Box<Expr>, usize),
    SETPENCOLOR(Box<Expr>, usize),
    TURN(Box<Expr>, usize),
    SETHEADING(Box<Expr>, usize),
    SETX(Box<Expr>, usize),
    SETY(Box<Expr>, usize),
    ADDASSIGN(Box<Assign>, Box<Expr>, usize),
    FUNC(Box<DeclName>, Vec<Box<Expr>>, usize),
    COMMENT(String, usize),
}

#[derive(Debug, PartialEq)]
pub struct Decl {
    pub name: Box<DeclName>,
    pub var: Vec<Box<Assign>>,
}

#[derive(Debug, PartialEq)]
pub enum Assign {
    VAR(String, usize, usize),
    ERROR(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum DeclName {
    STRING(String, usize, usize),
    ERROR(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Box<Assign>>,
    pub stmt_list: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    map: HashMap<String, FunctionType>,
}

impl Default for Function {
    fn default() -> Self {
        Self::new()
    }
}

impl Function {
    pub fn new() -> Self {
        Function {
            map: HashMap::new(),
        }
    }

    pub fn check_name(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    pub fn insert(&mut self, name: String, args: Vec<Box<Assign>>, stmt_list: Vec<Stmt>) {
        self.map.insert(name, FunctionType { args, stmt_list });
    }

    pub fn get(&self, name: &str) -> Option<&FunctionType> {
        self.map.get(name)
    }

    pub fn get_main(&self) -> &Vec<Stmt> {
        self.map.get("").unwrap().stmt_list.as_ref()
    }

    pub fn get_args_value(&self, name: &str) -> Option<usize> {
        self.map.get(name).map(|value| value.args.len())
    }

    pub fn get_args_by_name(&self, name: &str) -> HashMap<String, bool> {
        let mut set = HashMap::new();
        self.map.get(name).unwrap().args.iter().for_each(|x| {
            if let Assign::VAR(name, ..) = x.as_ref() {
                set.insert(name.to_string(), false);
            }
        });
        set
    }

    pub fn get_args(&self) -> Vec<String> {
        let mut list = vec![];
        for arg in self.map.values() {
            for a in arg.args.iter() {
                if let Assign::VAR(name, ..) = a.as_ref() {
                    list.push(name.to_string());
                }
            }
        }
        list
    }

    pub fn get_all(&self) -> &HashMap<String, FunctionType> {
        &self.map
    }
}
