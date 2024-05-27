#![allow(non_snake_case, unused)]

use std::{fs::File, io::{BufReader, Read}, path::Path};
use anyhow::{anyhow, Result};

const MAGIC_STRING: &str= "\0asm";

enum WasmBinaryVersion {
  One = 1
}

// The distributable, loadable, and executable unit of code in WebAssembly is called a module. A
// module collects definitions for types, functions, tables, memories, and globals. In addition, it
// can declare imports and exports and provide initialization.
// Reference : https://github.com/WebAssembly/design/blob/main/Modules.md.
struct WasmModule {
  binaryVersion: WasmBinaryVersion
}

impl WasmModule {
  // Constructs an instance of the WasmModule struct by (buffered) reading the given WASM module.
  // The instance is then returned.
  fn new(path: &str) -> Result<Self> {
    let mut file = File::open(Path::new(path))?;

    let mut wasmModuleReader= WasmModuleReader(

      // NOTE : Default buffer size is 8 KB.
      BufReader::new(file)
    );
    wasmModuleReader.read( )
  }
}

struct WasmModuleReader(BufReader<File>);

impl WasmModuleReader {
  // Constructs an instance of the WasmModule struct by (buffered) reading the given WASM module.
  // The instance is then returned.
  fn read(&mut self) -> Result<WasmModule> {

    /*
      The encoding of a module starts with a preamble containing :
        (1) a 4-byte magic string (\0asm).
        (2) a version field. The current version of the WASM binary format is 1.
    */
    let wasmBinaryVersion= self.readPreamble( )?;

    Ok(WasmModule {
      binaryVersion: wasmBinaryVersion
    })
  }

  // Reads the preamble in the WASM module and validates the WASM binary version. Returns the WASM
  // binary version.
  fn readPreamble(&mut self) -> Result<WasmBinaryVersion> {
    let magicString= self.readBytesAsString(4)?;
    if magicString != MAGIC_STRING {
      return Err(anyhow!("Magic string not found"))}

    let wasmBinaryVersion= self.readBytesAsU32(4)?;
    if wasmBinaryVersion != WasmBinaryVersion::One as u32 {
      return Err(anyhow!("Wrong WASM binary version"))}

    return Ok(WasmBinaryVersion::One)
  }
}

impl WasmModuleReader {
  // Reads and returns the given number of bytes from the WASM module.
  fn readBytes(&mut self, byteCount: usize) -> Result<Vec<u8>> {
    let mut buffer= vec![0u8; byteCount];
    self.0.read(&mut buffer)?;
    Ok(buffer)
  }

  // Reads and returns the given number of bytes (as UTF8 string) from the WASM module.
  fn readBytesAsString(&mut self, byteCount: usize) -> Result<String> {
    let mut stringBuffer= String::new( );
    self.0.read_to_string(&mut stringBuffer)?;
    Ok(stringBuffer)
  }

  // Reads and returns the given number of bytes (as u32) from the WASM module.
  fn readBytesAsU32(&mut self, byteCount: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
      self.readBytes(byteCount)?
        .as_slice( ).try_into( )?
    ))
  }
}

fn main( ) {
  let wasmModule= WasmModule::new("./test/test.wasm");
}
