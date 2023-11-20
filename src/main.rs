use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

enum WasmOutput {
    PtrLen,
    TinyGoMalloc
}

struct OutputMap {
    output: WasmOutput,
    ptr: Option<String>,
    len: Option<String>,
}

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

fn run_mem_wasm(filename: String, output_map: OutputMap) -> Result<()> {
    let output_type = output_map.output;
    let ptr_output = match output_map.ptr {
        Some(ptr) => ptr,
        None => String::from("output_ptr")
    };
    let len_output = match output_map.len {
        Some(len) => len,
        None => String::from("output_len")
    };

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

    //Get length
    let len = {
        let get_output_len = instance.get_func(&mut store, &len_output).expect("Couldn't get the output length function - did you specify it? If so, does it exist? If not, does the default `output_len` exist?");
        let mut result = [wasmtime::Val::I32(0)];
        get_output_len.call(&mut store, &[], &mut result).expect("Couldn't call the output length function.");
        let output_len = result[0].unwrap_i32();

        output_len
    };

    //Get output ptr
    let ptr = match output_type {
        WasmOutput::PtrLen => {
            let get_output_ptr = instance.get_func(&mut store, &ptr_output).expect("Couldn't get the output pointer function - did you specify it? If so, does it exist? If not, does the default `output_ptr` exist?");
            let mut result = [wasmtime::Val::I32(0)];
            get_output_ptr.call(&mut store, &[], &mut result).expect("Couldn't call the output pointer function.");
            let output_ptr = result[0].unwrap_i32();

            output_ptr
        },
        WasmOutput::TinyGoMalloc => {
            //malloc params: size of result (1 32-bit integer -- it's just a pointer)
            //malloc results (where to store value in WASM memory): &mut 32-bit integer as pointer (offset in memory)
            let malloc = instance.get_func(&mut store, "malloc").expect("Couldn't get malloc from the TinyGo WASM module.");
            let mut result = [wasmtime::Val::I32(0)];
            malloc.call(&mut store, &[wasmtime::Val::I32(2)], &mut result).expect("Couldn't call TinyGo's malloc function.");
            let output_ptr = result[0].unwrap_i32();
            
            output_ptr
        }
    };

    println!("Output ptr: {}; output length: {}", ptr, len);

    let mut buf: Vec<u8> = vec![0u8; len as usize];
    memory.read(&store, ptr as usize, &mut buf).unwrap();

    println!("Output: {}", String::from_utf8(buf).unwrap());

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
        run_mem_wasm(String::from("go/main.wasm"), OutputMap { 
            output: WasmOutput::TinyGoMalloc,
            ptr: None,
            len: None
        }).unwrap();
    }

    #[test]
    fn test_run_rust_wasm() {
        run_mem_wasm(String::from("rust/main.wasm"), OutputMap { 
            output: WasmOutput::PtrLen,
            ptr: None,
            len: None
        }).unwrap();
    }
}
