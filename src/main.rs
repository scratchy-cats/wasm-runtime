#![allow(non_snake_case)]

use std::{fs::File, io::Read, path::Path};
use module::Module;

mod module;

fn main( ) {
  let modulePath= "./test/test.wasm";
  // dumpModuleAsHexString(modulePath);

  let _= Module::new(&modulePath).unwrap( );
}

// Prints out content of the WASM module at the given path, as hexadecimal string.
// NOTE : This is for quick debugging purposes.
#[allow(unused)]
fn dumpModuleAsHexString(path: &str) {
  let mut file = File::open(Path::new(path)).unwrap( );

  let mut buffer = Vec::new( );
  file.read_to_end(&mut buffer).unwrap( );

  let contentAsHexString: String = buffer.iter( )
    .map(|byte| format!("{:02x}", byte))
    .collect( );

  println!("WASM module contents as hexadecimal string : {}", contentAsHexString);
}

/*
  Analysing the WASM Module binary :

  0061736d - Magic String
  01000000 - WASM Binary Verson

  01 - Type Section
  07 - Section content byte-size
  01 - Function signatures count

    60 - Function Signature starting marker

      02 - Input count.
        7f - Number type.
        7f - Number type.

      01 - Output count.
        7f - Number type.
  
  030201000a09010700200020016a0b
*/
