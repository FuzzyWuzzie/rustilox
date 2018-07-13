use values::Value;
use errors::LoxError;
use compiler::compile;
use chunk::Chunk;
use vm::VM;

pub fn interpret(vm: &mut VM, source: &str) -> Result<Value, LoxError> {
    let mut chunk = Chunk::init();

    if let Err(e) = compile(source, &mut chunk) {
        return Err(e);
    }

    vm.ip = 0;
    vm.chunk = Box::new(chunk);

    vm.evaluate()
}