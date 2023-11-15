use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

fn main() {
    println!("Hello, world!");
    run_wasm(String::from("./wasm/hello.wasm")).unwrap();
}

// fn get_string_from_wasm_memory(mem: &Memory, store: &mut wasmtime::Store<AppState>, offset: i32, len: i32) -> Vec<u8> {
//     let mut buffer = vec![0u8; len as usize];
//     match mem.read(&store, offset as usize, &mut buffer) {
//         Ok(_) => buffer,
//         Err => vec![],
//     }
// }

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
            wasi,
        }
    );
    let module = Module::from_file(&engine, "go/main.wasm")?;

    //Instantiate the module
    println!("Instantiating module...");  
    let instance = linker.instantiate(&mut store, &module)?;

    //WASM memory
    let memory = instance
        .get_memory(&mut store, "memory")
        .unwrap();

    //Extract export
    println!("Extracting export...");
    // let run = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    // let run_str = instance.get_typed_func::<(), (i32, i32)>(&mut store, "retStr").unwrap();
    // let run_ptr = instance.get_typed_func::<(), i32>(&mut store, "html_ptr")?;
    let greet = instance
        .get_typed_func::<(), String>(&mut store, "retStr")
        .expect("retStr wasn't a function")
        .get0();

    //Call export
    println!("Calling export...");
    let val = greet.call(&mut store, &[], &mut [])?;
    println!("Result: {:?}", val);
    // let val = run.call(&mut store, (12, 8)).expect("Should have got a number");
    // println!("Result: {}", val);

    // // let strptr = run_ptr.call(&mut store, ())?;

    // let str_result = run_str.call(&mut store,()).unwrap();
    // // println!("Result length: {}", strlen);
    // let ptr = str_result.0;
    // let len = str_result.1;

    // let strval = get_string_from_wasm_memory(&memory, &mut store, 1, strlen);
    // println!("Result: {}", String::from_utf8(strval).unwrap());
    // println!("Result: {}, len {}", ptr, len);

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
