extern crate inkwell;

use inkwell::{OptimizationLevel, AddressSpace};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Module, Linkage};

use std::error::Error;
use inkwell::targets::{CodeModel, RelocMode, TargetTriple, FileType};
use inkwell::types::IntType;
use inkwell::values::{PointerValue, FunctionValue};
use inkwell::passes::PassManager;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx, 'a> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    builder: Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
}

impl<'ctx, 'a> CodeGen<'ctx, 'a> {
    fn emit_global_string(&self, string: &&str, name: &str) -> PointerValue {
        let ty = self.context.i8_type().array_type(string.len() as u32);
        let gv = self
            .module
            .add_global(ty, Some(AddressSpace::Generic), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&self.context.const_string(string.as_ref(), false));

        let pointer_value = self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        );

        pointer_value
    }

    fn emit_printf_call(&self, hello_str: &&str, name: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        let printf = self
            .module
            .add_function("out", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(hello_str, name);
        self.builder.build_call(printf, &[pointer_value.into()], "call");

        i32_type
    }

    fn compile(&self) {
        let i32_type = self.context.i32_type();
        let function_type = i32_type.fn_type(&[], false);

        let function = self.module.add_function("main", function_type, Some(Linkage::External));
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let i32_type = self.emit_printf_call(&"hello, world!\n", "hello");
        self.builder
            .build_return(Some(&i32_type.const_int(0, false)));

        self.fpm.run_on(&function);
    }
}

fn main() {
    let context = Context::create();
    let module = context.create_module("repl");

    // Create FPM
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let codegen = CodeGen {
        context: &context,
        module: &module,
        builder: context.create_builder(),
        fpm: &fpm,
    };

    codegen.compile();

    inkwell::targets::Target::initialize_webassembly(&Default::default());
    let target = inkwell::targets::Target::from_name("wasm32").unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetTriple::create("wasm32-unknown-emscripten"),
            "",
            "",
            OptimizationLevel::Aggressive,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    target_machine.write_to_file(&codegen.module, FileType::Object, "hello.wasm".as_ref());
}

