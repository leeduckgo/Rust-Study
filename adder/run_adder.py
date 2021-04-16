from wasmer import engine, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

# Let's define the store, that holds the engine, that holds the compiler.
store = Store(engine.JIT(Compiler))

# Let's compile the module to be able to execute it!
module = Module(store, open('target/wasm32-wasi/release/adder.wasm', 'rb').read())

# Now the module is compiled, we can instantiate it.
instance = Instance(module)

# Call the exported `sum` function.
result = instance.exports.adder(5, 37)

print(result) # 42!
