use anyhow::Result;
use wasmtime::*;

fn main() {
    println!("Hello, world!");
}

fn run_wasm()-> Result<()> {
    //Load wasm from disk
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "wasm/hello.wat")?;

    //Instantiate the module
    println!("Instantiating module...");
    let mut store = Store::new(
        &engine,
        ()
    );

    //Create a callback
    let hello_func = Func::wrap(&mut store, |_caller: Caller<'_, ()>| {
        println!("Calling back...");
    });
    
    //Create an import object
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    //Extract export
    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    //Call export
    println!("Calling export...");
    run.call(&mut store, ())?;

    println!("Done.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_wasm() {
        run_wasm().unwrap();
    }
}