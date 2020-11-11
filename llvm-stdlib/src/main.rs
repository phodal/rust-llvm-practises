use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::OptimizationLevel;

static CHARJ_LIB: &[u8] = include_bytes!("../stdlib/charj.bc");

fn main() {
    let context = Context::create();
    let module = load_std_lib(&context);

    match module.get_function("main") {
        None => { println!("none"); }
        Some(_fun) => {
            let ee = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
            let maybe_fn = unsafe {
                ee.get_function::<unsafe extern "C" fn() -> f64>("main")
            };

            unsafe {
                maybe_fn.unwrap().call();
            }
        }
    }
}

pub fn load_std_lib(context: &Context) -> Module {
    let memory = MemoryBuffer::create_from_memory_range(CHARJ_LIB, "solana");
    let module = Module::parse_bitcode_from_buffer(&memory, context).unwrap();
    return module;
}
