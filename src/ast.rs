pub enum Type {
    I8,
    I16,
    I32,
    I64,
}

pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
    Lsh,
    Rsh,
}

pub struct BinaryExpr {
    pub op: BinaryOp,
    pub a: Box<Expr>,
    pub b: Box<Expr>,
}

pub enum UnaryOp {
    Deref,
    Not
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub a: Box<Expr>,
}

pub struct InvokeExpr {
    pub func_name: String,
    pub params: Vec<Expr>,
}

pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

pub struct AssignmentExpr {
    pub local: u32,
    pub val: Box<Expr>,
}

pub enum Expr {
    Binary(BinaryExpr),
    Invoke(InvokeExpr),
    Block(BlockExpr),
    Assignment(AssignmentExpr),
}

pub enum Linkage {
    Internal,
    External
}

pub struct FunctionDeclItem {
    pub name: String,
    pub return_ty: Type,
    pub params: Vec<Type>,
    pub linkage: Linkage
}

pub struct FunctionDefItem {
    pub name: String,
    pub locals: Vec<Type>,
    pub code: Expr,
}

pub enum Item {
    FunctionDecl(FunctionDeclItem),
    FunctionDef(FunctionDefItem),
}
