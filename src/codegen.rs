use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{IntValue, BasicValueEnum};
use inkwell::AddressSpace;

use crate::ast;

fn compile(ast: Vec<ast::Item>) {
    let context = Context::create();
    let module = context.create_module("stack_bad");
    let codegen = Codegen {
        context: &context,
        module,
        builder: context.create_builder()
    };
}

struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    fn build_expr(&mut self, expr: ast::Expr) -> IntValue<'ctx> {
        match expr {
            ast::Expr::Binary(expr) => {
                let a = self.build_expr(*expr.a);
                let b = self.build_expr(*expr.b);
                match expr.op {
                    ast::BinaryOp::Add => {
                        self.builder.build_int_add(a, b, "")
                    }
                    ast::BinaryOp::Sub => {
                        self.builder.build_int_sub(a, b, "")
                    }
                    ast::BinaryOp::Mult => {
                        self.builder.build_int_mul(a, b, "")
                    }
                    ast::BinaryOp::Div => {
                        self.builder.build_int_signed_div(a, b, "")
                    }
                    ast::BinaryOp::Lsh => {
                        self.builder.build_left_shift(a, b, "")
                    }
                    ast::BinaryOp::Rsh => {
                        self.builder.build_right_shift(a, b, true, "")
                    }
                }
            }
            ast::Expr::Unary(expr) => {
                let a = self.build_expr(*expr.a);
                match expr.op {
                    ast::UnaryOp::Deref => {
                        let ptr_type = self.context.i32_type().ptr_type(AddressSpace::Generic);
                        let ptr = self.builder.build_int_to_ptr(a, ptr_type, "");
                        match self.builder.build_load(ptr, "") { 
                            BasicValueEnum::IntValue(val) => val,
                            _ => panic!("stack bad")
                        }
                    }
                    ast::UnaryOp::Not => {
                        self.builder.build_not(a, "")
                    }
                }
            }
            ast::Expr::Invoke(expr) => {
                let fn_val = self.module.get_function(&expr.func_name).unwrap();
                let args: Vec<BasicValueEnum> = expr.params.into_iter().map(|p| BasicValueEnum::IntValue(self.build_expr(p))).collect();
                let val = self.builder.build_call(fn_val, &args, "").try_as_basic_value().left().unwrap();
                match val {
                    BasicValueEnum::IntValue(val) => val,
                    _ => panic!("stack bad")
                }
            }
            ast::Expr::Block(expr) => {
                todo!();
            }
            ast::Expr::Assignment(expr) => {
                todo!();
                
            }
            ast::Expr::Local(expr) => {
                todo!();
            }
            ast::Expr::Constant(expr) => {
                self.context.i64_type().const_int(expr.val as u64, false)
            }
            ast::Expr::Return(expr) => {
                let val = self.build_expr(*expr.val);
                self.builder.build_return(Some(&val));
                self.context.i64_type().const_int(0, false)
            }
        }
    }
}
