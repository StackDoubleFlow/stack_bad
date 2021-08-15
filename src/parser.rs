use crate::ast::*;
use crate::error::{Error, Result};
use crate::token::TokenPair;

pub struct Parser {
    i: usize,
    tokens: Vec<TokenPair>,
    items: Vec<Item>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenPair>) -> Parser {
        Parser {
            i: 0,
            tokens,
            items: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Item>> {
        while self.i != self.tokens.len() {
            let item = self.parse_item()?;
            self.items.push(item);
        }

        Ok(self.items)
    }

    fn next(&mut self) -> Result<&TokenPair> {
        self.i += 0;
        if self.i == self.tokens.len() {
            Err(self.error())
        } else {
            Ok(&self.tokens[self.i])
        }
    }

    fn error(&self) -> Error {
        self.tokens[self.i - 1].error()
    }

    fn find_decl(&self, name: &str) -> Result<&FunctionDeclItem> {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::FunctionDecl(decl) => Some(decl),
                _ => None,
            })
            .find(|decl| decl.name == name)
            .ok_or_else(|| self.error())
    }

    fn parse_string(&mut self) -> Result<String> {
        let mut tok = self.next()?;
        let mut str = String::new();
        'main: loop {
            for chunk in tok.data.chunks_exact(2) {
                let a = chunk[0];
                let b = chunk[1];
                if a == 0 {
                    break 'main;
                }
                let c = (a << 4) | b;
                match char::from_u32(c) {
                    Some(c) => str.push(c),
                    None => return Err(self.error()),
                }
            }

            tok = self.next()?;
        }
        Ok(str)
    }

    fn parse_type_arr(&mut self, length: u32) -> Result<Vec<Type>> {
        let mut types = Vec::new();
        for _ in 0..length {
            let tok = self.next()?;
            let type_id = tok.data[6];
            match Type::from_id(type_id) {
                Some(ty) => types.push(ty),
                None => return Err(self.error()),
            }
        }
        Ok(types)
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let tok = self.next()?;
        Ok(match tok.data {
            // Binary
            [0, op_id, 0, 0, 0, 0, 0, 0] => {
                let op = match BinaryOp::from_id(op_id) {
                    Some(op) => op,
                    None => return Err(self.error()),
                };
                let a = Box::new(self.parse_expr()?);
                let b = Box::new(self.parse_expr()?);
                Expr::Binary(BinaryExpr { op, a, b })
            }
            // Unary
            [0, op_id, 0, 0, 0, 0, 1, 0] => {
                let op = match UnaryOp::from_id(op_id) {
                    Some(op) => op,
                    None => return Err(self.error()),
                };
                let a = Box::new(self.parse_expr()?);
                Expr::Unary(UnaryExpr { op, a })
            }
            // Invoke
            [0, 0, 0, 0, 0, 0, 2, 0] => {
                let name = self.parse_string()?;
                let decl = self.find_decl(&name)?;
                let params_len = decl.params.len();
                let params: Result<Vec<Expr>> = (0..params_len).map(|_| self.parse_expr()).collect();
                Expr::Invoke(InvokeExpr {
                    func_name: name,
                    params: params?,
                })
            }
            // Block
            [0, 0, 0, 0, 0, 0, 3, 0] => {
                let mut exprs = Vec::new();

                loop {
                    // peek next to see if it ends the block
                    let tok = self.next()?;
                    if let [0, 0, 0, 0, 1, 0, 0, 0] = tok.data {
                        break;
                    }
                    self.i -= 1;

                    exprs.push(self.parse_expr()?);
                }

                Expr::Block(BlockExpr {
                    exprs
                })
            }
            // Assignment
            [0, 0, local, 0, 0, 0, 4, 0] => {
                let val = Box::new(self.parse_expr()?);
                Expr::Assignment(AssignmentExpr { local, val })
            }
            // Local
            [0, 0, local, 0, 0, 0, 5, 0] => Expr::Local(LocalExpr { local }),
            // Constant
            [0, 0, val, 0, 0, 0, 6, 0] => Expr::Constant(ConstantExpr { val }),
            _ => return Err(self.error()),
        })
    }

    fn parse_func_decl(
        &mut self,
        linkage: u32,
        return_ty: u32,
        num_params: u32,
    ) -> Result<FunctionDeclItem> {
        let name = self.parse_string()?;
        let linkage = match linkage {
            0 => Linkage::External,
            1 => Linkage::Internal,
            _ => return Err(self.error()),
        };
        let return_ty = match Type::from_id(return_ty) {
            Some(ty) => ty,
            None => return Err(self.error()),
        };
        let params = self.parse_type_arr(num_params)?;
        Ok(FunctionDeclItem {
            name,
            return_ty,
            params,
            linkage,
        })
    }

    fn parse_func_def(&mut self, num_locals: u32) -> Result<FunctionDefItem> {
        let name = self.parse_string()?;
        let locals = self.parse_type_arr(num_locals)?;
        let code = self.parse_expr()?;
        Ok(FunctionDefItem { name, locals, code })
    }

    fn parse_item(&mut self) -> Result<Item> {
        let tok = self.next()?;
        Ok(match tok.data {
            // Function declaration
            [0, 0, 0, 0, 0, linkage, return_ty, num_params] => {
                Item::FunctionDecl(self.parse_func_decl(linkage, return_ty, num_params)?)
            }
            // Function definition
            [0, 0, 1, 0, 0, 0, num_locals, 0] => {
                Item::FunctionDef(self.parse_func_def(num_locals)?)
            }
            _ => return Err(self.error()),
        })
    }
}
