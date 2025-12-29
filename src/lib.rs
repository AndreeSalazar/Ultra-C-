pub mod codegen;
pub mod parser;
pub mod tool_detector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub vis: Visibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    LiteralString(String),
    LiteralInt(i64),
    LiteralBool(bool),
    LiteralFloat(f64),
    UnaryOp(String, Box<Expr>),
    SelfField(String),
    Variable(String),
    SelfCall {
        name: String,
        args: Vec<Expr>,
    },
    SuperCall {
        name: String,
        args: Vec<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Block(Vec<Expr>),
    If {
        cond: Box<Expr>,
        then_body: Box<Expr>,
        else_body: Option<Box<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    Return(Option<Box<Expr>>),
    VarDecl {
        name: String,
        ty: String,
        value: Option<Box<Expr>>,
    },
    Concat(Box<Expr>, Box<Expr>),
    Native(String),
    FileCall(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub params: Vec<Param>,
    pub body: Expr,
    pub is_static: bool,
    pub vis: Visibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub base: Option<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub ctor_params: Option<Vec<Param>>,
    pub ctor_body: Option<Expr>,
    pub extra_includes: Vec<String>,
    pub namespace: Option<String>,
    pub module_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Directives {
    pub uses: Vec<String>,
    pub profiles: Vec<String>,
    pub capabilities: Vec<String>,
    pub entry: Option<String>,
    pub global_base: bool,
    pub imports: Vec<String>,
    pub namespace: Option<String>,
}

pub fn resolve_includes(d: &Directives) -> Vec<String> {
    let mut set: Vec<String> = Vec::new();
    let add = |set: &mut Vec<String>, s: &str| {
        if !set.iter().any(|x| x == s) {
            set.push(s.to_string());
        }
    };
    for p in &d.profiles {
        match p.as_str() {
            "std" => {
                add(&mut set, "string");
                add(&mut set, "memory");
                add(&mut set, "iostream");
                add(&mut set, "vector");
                add(&mut set, "algorithm");
                add(&mut set, "functional");
            }
            "math" => {
                add(&mut set, "cmath");
                add(&mut set, "numeric");
            }
            _ => {}
        }
    }
    for u in &d.uses {
        match u.as_str() {
            "std" => {
                add(&mut set, "string");
                add(&mut set, "memory");
                add(&mut set, "iostream");
                add(&mut set, "vector");
                add(&mut set, "algorithm");
            }
            "std::io" | "io" => add(&mut set, "iostream"),
            "std::string" | "string" => add(&mut set, "string"),
            "std::vector" | "vector" => add(&mut set, "vector"),
            "std::map" | "map" => add(&mut set, "map"),
            "std::unordered_map" | "unordered_map" => add(&mut set, "unordered_map"),
            "std::optional" | "optional" => add(&mut set, "optional"),
            "std::algorithm" | "algorithm" => add(&mut set, "algorithm"),
            "std::functional" | "functional" => add(&mut set, "functional"),
            _ => {}
        }
    }
    for c in &d.capabilities {
        match c.as_str() {
            "io" => add(&mut set, "iostream"),
            "string" => add(&mut set, "string"),
            "vector" => add(&mut set, "vector"),
            "map" => add(&mut set, "map"),
            "unordered_map" => add(&mut set, "unordered_map"),
            "optional" => add(&mut set, "optional"),
            "algorithm" => add(&mut set, "algorithm"),
            _ => {}
        }
    }
    set
}
