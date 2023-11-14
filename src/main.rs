use wasmtime::{*, component::WasmStr};
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

fn main() {
    println!("Hello, world!");
    run_wasm(String::from("./wasm/hello.wasm")).unwrap();
}

fn run_wasm(filename: String) -> Result<()> {
    //Load wasm from disk
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, filename)?;

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

struct AppState {
    message: String,
    wasi: WasiCtx,
}

fn run_go_wasm() -> Result<()> {
    //Load wasm from disk
    println!("Compiling module...");
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut AppState| &mut state.wasi)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(
        &engine,
        AppState {
            message: "Hello from Rust!".to_string(),
            wasi,
        }
    );
    let module = Module::from_file(&engine, "go/main.wasm")?;

    //Instantiate the module
    println!("Instantiating module...");  
    let instance = linker.instantiate(&mut store, &module)?;

    //Extract export
    println!("Extracting export...");
    let run = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let run_str = instance.get_typed_func::<(), (i32, i32)>(&mut store, "retStr")?;

    //Call export
    println!("Calling export...");
    let val = run.call(&mut store, (12, 8)).expect("Should have got a number");
    println!("Result: {}", val);

    let strval = run_str.call(&mut store, ());
    println!("Result: {}", strval);

    println!("Done.");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_wat() {
        run_wasm(String::from("./wasm/hello.wat")).unwrap();
    }

    #[test]
    fn test_run_wasm() {
        run_wasm(String::from("./wasm/hello.wasm")).unwrap();
    }

    #[test]
    fn test_run_go_wasm() {
        run_go_wasm().unwrap();
    }
}
