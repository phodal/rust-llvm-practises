extern crate inkwell;

use inkwell::{AddressSpace, OptimizationLevel};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::passes::PassManager;
use inkwell::targets::{CodeModel, FileType, RelocMode, TargetTriple};
use inkwell::values::{FunctionValue, PointerValue};

struct CodeGen<'ctx, 'a> {
    context: &'ctx Context,
    module: Module<'ctx>,
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

    fn compile(&self) {
        let function_type = self.context.void_type().fn_type(&[], false);

        // create function & block
        let main_func = self.module.add_function("main", function_type, None);
        let main_block = self.context.append_basic_block(main_func, "main");
        self.builder.position_at_end(main_block);

        // create inner out function
        let printf_func = self.create_out_func();

        // make call
        let pointer_value = self.emit_global_string(&"hello, world!", "");
        self.builder.build_call(printf_func, &[pointer_value.into()], "");

        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));

        self.fpm.run_on(&main_func);
    }

    fn create_out_func(&self) -> FunctionValue {
        if let Some(fun) = self.module.get_function("out") {
            return fun
        }

        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = self.context.i32_type().fn_type(&[str_type.into()], true);
        let printf_func = self
            .module
            .add_function("out", printf_type, Some(Linkage::External));
        printf_func
    }
}

fn main() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

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

    let codegen = CodeGen { context: &context, module, builder, fpm: &fpm };
    codegen.compile();

    inkwell::targets::Target::initialize_webassembly(&Default::default());
    let target = inkwell::targets::Target::from_name("wasm32").unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetTriple::create("wasm32-unknown-unknown-wasm"),
            "",
            "",
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    let _result = target_machine.write_to_file(&codegen.module, FileType::Object, "hello.wasm".as_ref());
}

