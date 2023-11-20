use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

fn main() {
    println!("Hello, world!");
    run_wasm(String::from("./wasm/hello.wasm")).unwrap();
}

// fn get_ptr_len(val: i64)-> (u8, usize) {
//     let ptr = ( val & 0xFFFFFFFF ) as u8;
//     let len = (val >> 32) as usize;
//     (ptr, len)
// }

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

#[derive(Debug)]
#[repr(C)]
struct GoStringParameters {
    ptr: i32,
    len: i32
}

fn run_mem_wasm(filename: String) -> Result<()> {
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
    
    let module = Module::from_file(&engine, filename)?;

    //Instantiate the module
    println!("Instantiating module...");  
    let instance = linker.instantiate(&mut store, &module)?;

    //WASM memory
    let memory = instance
        .get_memory(&mut store, "memory")
        .unwrap();

    println!("Memory size: {}", memory.size(&store));



    //Grow memory by 2 bytes
    // memory.grow(&mut store, 50).expect("Grow memory");
    // println!("Memory size: {}", memory.size(&store));

    // let len = 52;
    // let mut buf = vec![0u8; len as usize];
    // memory.read(&store, 2, &mut buf).unwrap();
    // println!("Memory: {:?}", buf);

    // let go_str_addr = {
    //     let malloc = instance.get_func(&mut store, "malloc").expect("Couldn't get malloc");
    //     let mut result = [wasmtime::Val::I32(0)];
    //     malloc.call(&mut store, &[wasmtime::Val::I32(mem::size_of::<GoStringParameters>() as i32)], &mut result).expect("Couldn't call malloc");
    //     result[0].unwrap_i32()
    // };

    //Extract export
    // println!("Extracting export...");
    // let wasm_return_string_function = instance.get_func(&mut store, "retStr").expect("Couldn't get retStr");
    // let run = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    // let run_str = instance.get_typed_func::<(), (i32, i32)>(&mut store, "retStr").unwrap();
    // let run_ptr = instance.get_typed_func::<(), i32>(&mut store, "html_ptr")?;
    // let greet = instance
    //     .get_typed_func::<(), i32>(&mut store, "retStr")
    //     .expect("retStr wasn't a function");

    //Call export
    // println!("Calling export...");
    // wasm_return_string_function.call(&mut store,  &[], &mut []).expect("Call retStr");
    // greet.call(&mut store, ())?;
    // let mut buf = vec![0u8; len as usize];
    // memory.read(&store, len, &mut buf).unwrap();
    // println!("Memory: {:?}", buf);
    
    // println!("Reading memory");
    // let mut buf = [0u8; mem::size_of::<GoStringParameters>()];
    // memory.read(&mut store, go_str_addr as usize, &mut buf).expect("Get WASM memory");
    // let go_str_parameters: GoStringParameters = unsafe { mem::transmute(buf) };
    // dbg!(&go_str_parameters);

    // let mut str_bytes = vec![0u8; go_str_parameters.len as usize];
    // memory.read(&mut store, go_str_parameters.ptr as usize, &mut str_bytes).expect("Read string bytes");
    // let rust_str = String::from_utf8(str_bytes).unwrap();
    // dbg!(rust_str);



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
        run_mem_wasm(String::from("go/main.wasm")).unwrap();
    }

    #[test]
    fn test_run_rust_wasm() {
        run_mem_wasm(String::from("rust/main.wasm")).unwrap();
    }
}
