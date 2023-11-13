use wasmtime::*;

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

fn run_go_wasm() -> Result<()> {
    //Load wasm from disk
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "go/main.wasm")?;

    //Instantiate the module
    println!("Instantiating module...");
    let mut store = Store::new(
        &engine,
        ()
    );

    //Create a callback
    // let print_result = Func::wrap(&mut store, |_caller: Caller<'_, ()>| {
    //     println!("Calling back...");
    // });
    
    //Create an import object
    // let imports = [print_result.into()];
    let instance = Instance::new(&mut store, &module, &[])?;

    //Extract export
    println!("Extracting export...");
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    //Call export
    println!("Calling export...");
    let val = add.call(&mut store, (12, 8))?;
    format!("Result: {}", val);

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
