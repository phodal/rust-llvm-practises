use inkwell::{AddressSpace, OptimizationLevel};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::IntType;
use inkwell::values::PointerValue;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'ctx Context, builder: &'a Builder<'ctx>, module: &'a Module<'ctx>) {
        let compiler = Compiler {
            context,
            builder,
            module,
        };

        compiler.compile();
    }

    pub fn compile(&self) {
        let i64_type = self.context.i64_type();
        let function_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

        let function = self.module.add_function("main", function_type, None);
        let basic_block = self.context.append_basic_block(function, "entrypoint");

        self.builder.position_at_end(basic_block);

        let i32_type = self.emit_printf_call(&"hello, world!\n", "hello");
        self.builder.build_return(Some(&i32_type.const_int(0, false)));

        let _result = self.module.print_to_file("main.ll");
        self.execute()
    }

    fn emit_printf_call(&self, hello_str: &&str, name: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        // `printf` is same to `puts`
        let printf = self.module.add_function("puts", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(hello_str, name);
        self.builder.build_call(printf, &[pointer_value.into()], "");

        i32_type
    }

    fn execute(&self) {
        let ee = self.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        let maybe_fn = unsafe {
            ee.get_function::<unsafe extern "C" fn() -> f64>("main")
        };

        let compiled_fn = match maybe_fn {
            Ok(f) => f,
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        unsafe {
            compiled_fn.call();
        }
    }

    fn emit_global_string(&self, string: &&str, name: &str) -> PointerValue {
        let ty = self.context.i8_type().array_type(string.len() as u32);
        let gv = self.module.add_global(ty, Some(AddressSpace::Generic), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&self.context.const_string(string.as_ref(), false));

        let pointer_value = self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        );

        pointer_value
    }
}

pub fn create_compiler() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    Compiler::new(&context, &builder, &module);
}

fn main() {
    create_compiler();
}
