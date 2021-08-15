#[derive(Debug)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    Unit,
}

impl Type {
    pub fn from_id(id: u32) -> Option<Type> {
        use Type::*;
        Some(match id {
            0 => I8,
            1 => I16,
            2 => I32,
            3 => I64,
            4 => Unit,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
    Lsh,
    Rsh,
}

impl BinaryOp {
    pub fn from_id(id: u32) -> Option<BinaryOp> {
        use BinaryOp::*;
        Some(match id {
            0 => Add,
            1 => Sub,
            2 => Mult,
            3 => Div,
            4 => Lsh,
            5 => Rsh,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub a: Box<Expr>,
    pub b: Box<Expr>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Deref,
    Not,
}

impl UnaryOp {
    pub fn from_id(id: u32) -> Option<UnaryOp> {
        use UnaryOp::*;
        Some(match id {
            0 => Deref,
            1 => Not,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub a: Box<Expr>,
}

#[derive(Debug)]
pub struct InvokeExpr {
    pub func_name: String,
    pub params: Vec<Expr>,
}

#[derive(Debug)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct AssignmentExpr {
    pub local: u32,
    pub val: Box<Expr>,
}

#[derive(Debug)]
pub struct LocalExpr {
    pub local: u32,
}

#[derive(Debug)]
pub struct ConstantExpr {
    pub ty: Type,
    pub val: u32,
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub val: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Invoke(InvokeExpr),
    Block(BlockExpr),
    Assignment(AssignmentExpr),
    Local(LocalExpr),
    Constant(ConstantExpr),
    Return(ReturnExpr),
}

#[derive(Debug)]
pub enum Linkage {
    External,
    Internal,
}

#[derive(Debug)]
pub struct FunctionDeclItem {
    pub name: String,
    pub return_ty: Type,
    pub params: Vec<Type>,
    pub linkage: Linkage,
}

#[derive(Debug)]
pub struct FunctionDefItem {
    pub name: String,
    pub locals: Vec<Type>,
    pub code: Expr,
}

#[derive(Debug)]
pub enum Item {
    FunctionDecl(FunctionDeclItem),
    FunctionDef(FunctionDefItem),
}
