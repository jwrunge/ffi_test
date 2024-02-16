use wasmtime::*;
use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn Hello() -> *const c_char;
}

#[repr(C)]
struct _GoString {
    p: *const c_char,
    n: isize,
}

fn main() {
    println!("Hello, world!");
    run_wasm(String::from("./wasm/hello.wasm")).unwrap();
}

fn run_wasm(filename: String) -> Result<()> {
    //Load wasm from disk
    println!("Compiling module...");
    let engine = Engine::default();                                 //Stores and configures global compilation settings
    let module = Module::from_file(&engine, filename)?;       //Module is compiled form of input wasm module (compiles wat or runs wasm)

    //Instantiate the module
    println!("Instantiating module...");
    let mut store = Store::new(&engine, ());                    //Store owns instances, functions, globals, etc -- all wasm items. Can store custom data or 
    let instance = Instance::new(&mut store, &module, &[])?;  //Instance - instantiates WASM with store and optional imports

    //Get data back
    let hello_func = instance.get_func(&mut store, "hello").expect("`hello` was not exported by the module.");
    let hello = hello_func.typed::<(), i32>(&mut store)?;
    let res = hello.call(&mut store, ())?;

    println!("Result: {}", res);

    let res2 = hello.call(&mut store, ())?;

    println!("Result: {}", res2);

    println!("Done.");
    Ok(())
}

fn ffi_test() {
    // let c_path = CString::new(path).expect("CString::new failed");
    // let ptr = c_path.as_ptr();
    // let go_string = GoString {
    //     p: ptr,
    //     n: c_path.as_bytes().len() as i64,
    // };
    let result = unsafe { Hello() };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("BAD");
    match string.is_empty() || string.starts_with("BAD") {
        true => println!("Error: {}", string),
        false => println!("Success: {}", string),
    }
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
    fn test_go_ffi() {
        ffi_test();
    }
}
