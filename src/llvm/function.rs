use inkwell::values::FunctionValue;
use crate::llvm::codegen::Codegen;

impl<'ctx>Codegen<'ctx>{
    
    pub fn get_function(&mut self, key : u32) -> FunctionValue<'ctx>{
        let mut current = 0;
        while key != self.func_assigner[current]{
            current += 1;
        }

        self.functions[current]
    }
}