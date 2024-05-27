#![allow(non_snake_case, unused)]

use std::{fs::File, io::Read, path::Path};

// The distributable, loadable, and executable unit of code in WebAssembly is called a module. A
// module collects definitions for types, functions, tables, memories, and globals. In addition, it
// can declare imports and exports and provide initialization.
// Reference : https://github.com/WebAssembly/design/blob/main/Modules.md.
struct WasmModule { }

impl WasmModule {
  // Reads WASM module present at the given path, returning its content as a hexadecimal string.
  fn read(path: &str) -> String {
    let mut file= File::open(Path::new(path))
                        .expect("Failed opening given WASM module");

    let mut buffer= vec![ ];
    file.read_to_end(&mut buffer)
      .expect("Failed reading given WASM module");

    buffer.iter( )
      .map(|byte| format!("{:02x}", byte))
      .collect( )
  }
}

fn main( ) {
  let contents= WasmModule::read("./test/test.wasm");
  println!("{}", contents);
}
