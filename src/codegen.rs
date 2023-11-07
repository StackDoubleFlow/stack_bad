use crate::ast::{self};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::{BasicMetadataTypeEnum, IntType};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::{AddressSpace, OptimizationLevel};
use std::path::Path;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    cur_function: Option<FunctionValue<'ctx>>,
    cur_vars: Vec<PointerValue<'ctx>>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn compile(ast: Vec<ast::Item>, output_path: &Path) {
        let context = Context::create();
        let module = context.create_module("stack_bad");
        let mut codegen = Codegen {
            context: &context,
            module,
            builder: context.create_builder(),

            cur_function: None,
            cur_vars: Vec::new(),
        };

        for item in ast {
            match item {
                ast::Item::FunctionDecl(decl) => {
                    codegen.decl_function(decl);
                }
                ast::Item::FunctionDef(def) => {
                    codegen.define_function(def);
                }
            }
        }

        codegen.write_object(output_path);
    }

    fn write_object(&self, path: &Path) {
        Target::initialize_x86(&InitializationConfig::default());
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let cpu = TargetMachine::get_host_cpu_name();
        let features = TargetMachine::get_host_cpu_features();
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let opt = OptimizationLevel::Default;
        let target_machine = target
            .create_target_machine(
                &triple,
                cpu.to_str().unwrap(),
                features.to_str().unwrap(),
                opt,
                reloc,
                model,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Object, path)
            .unwrap();
    }

    fn decl_function(&self, decl: ast::FunctionDeclItem) {
        let ret_type = self.get_type_from_type(decl.return_ty);
        let param_types: Vec<BasicMetadataTypeEnum> = decl
            .params
            .into_iter()
            .map(|p| BasicMetadataTypeEnum::IntType(self.get_type_from_type(p)))
            .collect();

        let fn_type = ret_type.fn_type(&param_types, false);
        self.module.add_function(
            &decl.name,
            fn_type,
            Some(match decl.linkage {
                ast::Linkage::External => Linkage::External,
                ast::Linkage::Internal => Linkage::Internal,
            }),
        );
    }

    fn define_function(&mut self, def: ast::FunctionDefItem) {
        let func_val = self.find_function(&def.name);
        self.cur_function = Some(func_val);

        let entry = self.context.append_basic_block(func_val, "entry");
        self.builder.position_at_end(entry);

        for arg in func_val.get_param_iter() {
            let alloca = self.builder.build_alloca(arg.get_type(), "").unwrap();
            self.builder.build_store(alloca, arg).unwrap();
            self.cur_vars.push(alloca);
        }

        for local in def.locals {
            let alloca = self
                .builder
                .build_alloca(self.get_type_from_type(local), "")
                .unwrap();
            self.cur_vars.push(alloca);
        }

        self.build_expr(def.code);

        if func_val.verify(true) {
        } else {
            unsafe {
                func_val.delete();
            }

            panic!("stack bad");
        }

        self.cur_function = None;
        self.cur_vars.clear();
    }

    fn get_type_from_type(&self, ty: ast::Type) -> IntType<'ctx> {
        match ty {
            ast::Type::I8 => self.context.i8_type(),
            ast::Type::I16 => self.context.i16_type(),
            ast::Type::I32 => self.context.i32_type(),
            ast::Type::I64 => self.context.i64_type(),
            _ => panic!(),
        }
    }

    fn find_function(&self, name: &str) -> FunctionValue<'ctx> {
        self.module.get_function(name).unwrap()
    }

    fn build_expr(&mut self, expr: ast::Expr) -> IntValue<'ctx> {
        match expr {
            ast::Expr::Binary(expr) => {
                let a = self.build_expr(*expr.a);
                let b = self.build_expr(*expr.b);
                match expr.op {
                    ast::BinaryOp::Add => self.builder.build_int_add(a, b, ""),
                    ast::BinaryOp::Sub => self.builder.build_int_sub(a, b, ""),
                    ast::BinaryOp::Mult => self.builder.build_int_mul(a, b, ""),
                    ast::BinaryOp::Div => self.builder.build_int_signed_div(a, b, ""),
                    ast::BinaryOp::Lsh => self.builder.build_left_shift(a, b, ""),
                    ast::BinaryOp::Rsh => self.builder.build_right_shift(a, b, true, ""),
                }
                .unwrap()
            }
            ast::Expr::Unary(expr) => {
                let a = self.build_expr(*expr.a);
                match expr.op {
                    ast::UnaryOp::Deref => {
                        let ptr_type = self.context.i32_type().ptr_type(Default::default());
                        let ptr = self.builder.build_int_to_ptr(a, ptr_type, "").unwrap();
                        self.builder
                            .build_load(ptr_type, ptr, "")
                            .unwrap()
                            .into_int_value()
                    }
                    ast::UnaryOp::Not => self.builder.build_not(a, "").unwrap(),
                }
            }
            ast::Expr::Invoke(expr) => {
                let fn_val = self.module.get_function(&expr.func_name).unwrap();
                let args: Vec<BasicMetadataValueEnum> = expr
                    .params
                    .into_iter()
                    .map(|p| BasicMetadataValueEnum::IntValue(self.build_expr(p)))
                    .collect();
                let val = self
                    .builder
                    .build_call(fn_val, &args, "")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();
                val.into_int_value()
            }
            ast::Expr::Block(expr) => {
                for expr in expr.exprs {
                    self.build_expr(expr);
                }
                self.context.i64_type().const_int(0, false)
            }
            ast::Expr::Assignment(expr) => {
                let val = self.build_expr(*expr.val);
                self.builder
                    .build_store(self.cur_vars[expr.local as usize], val)
                    .unwrap();
                self.context.i64_type().const_int(0, false)
            }
            ast::Expr::Local(expr) => {
                let pointer_val = self.cur_vars[expr.local as usize];
                self.builder
                    .build_load(pointer_val.get_type(), pointer_val, "")
                    .unwrap()
                    .into_int_value()
            }
            ast::Expr::Constant(expr) => {
                let ty = self.get_type_from_type(expr.ty);
                ty.const_int(expr.val as u64, false)
            }
            ast::Expr::Return(expr) => {
                let val = self.build_expr(*expr.val);
                self.builder.build_return(Some(&val)).unwrap();
                self.context.i64_type().const_int(0, false)
            }
            ast::Expr::StringLit(expr) => {
                let elem_type = self.context.i8_type();
                let ty = elem_type.array_type(expr.str.len() as u32);
                // const address space
                let global = self.module.add_global(ty, Some(AddressSpace::from(4)), "");
                let items: Vec<_> = expr
                    .str
                    .bytes()
                    .map(|b| elem_type.const_int(b as u64, false))
                    .collect();
                let val = elem_type.const_array(&items);
                global.set_initializer(&val);
                let ptr = global.as_pointer_value();
                self.builder
                    .build_ptr_to_int(ptr, self.context.i64_type(), "")
                    .unwrap()
            }
        }
    }
}
