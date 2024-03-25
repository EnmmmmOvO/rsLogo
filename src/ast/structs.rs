use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Boolean(bool, usize, usize),
    Float(f32, usize, usize),
    Var(String, usize, usize),
    Add(Box<Expr>, Box<Expr>, usize, usize),
    Sub(Box<Expr>, Box<Expr>, usize, usize),
    Mul(Box<Expr>, Box<Expr>, usize, usize),
    Div(Box<Expr>, Box<Expr>, usize, usize),
    Eq(Box<Expr>, Box<Expr>, usize, usize),
    Ne(Box<Expr>, Box<Expr>, usize, usize),
    Lt(Box<Expr>, Box<Expr>, usize, usize),
    Gt(Box<Expr>, Box<Expr>, usize, usize),
    And(Box<Expr>, Box<Expr>, usize, usize),
    Or(Box<Expr>, Box<Expr>, usize, usize),
    XCor(usize, usize),
    YCor(usize, usize),
    Heading(usize, usize),
    Color(usize, usize),
    Error(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    If(Box<Expr>, Vec<Stmt>, usize),
    While(Box<Expr>, Vec<Stmt>, usize),
    Make(Box<Assign>, Box<Expr>, usize),
    PenUp(usize),
    PenDown(usize),
    Forward(Box<Expr>, usize),
    Back(Box<Expr>, usize),
    Left(Box<Expr>, usize),
    Right(Box<Expr>, usize),
    SetPenColor(Box<Expr>, usize),
    Turn(Box<Expr>, usize),
    SetHeading(Box<Expr>, usize),
    SetX(Box<Expr>, usize),
    SetY(Box<Expr>, usize),
    AddAssign(Box<Assign>, Box<Expr>, usize),
    Func(Box<DeclName>, Vec<Expr>, usize),
    Comments(String, usize),
}

#[derive(Debug, PartialEq)]
pub struct Decl {
    pub name: Box<DeclName>,
    pub var: Vec<Assign>,
}

#[derive(Debug, PartialEq)]
pub enum Assign {
    Var(String, usize, usize),
    Error(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum DeclName {
    String(String, usize, usize),
    Error(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Assign>,
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

    pub fn insert(&mut self, name: String, args: Vec<Assign>, stmt_list: Vec<Stmt>) {
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
            if let Assign::Var(name, ..) = &x {
                set.insert(name.to_string(), false);
            }
        });
        set
    }

    pub fn get_args(&self) -> Vec<String> {
        let mut list = vec![];
        for arg in self.map.values() {
            for a in arg.args.iter() {
                if let Assign::Var(name, ..) = &a {
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
