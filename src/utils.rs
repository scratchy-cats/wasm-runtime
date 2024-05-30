use std::{fs::File, io::Read, path::Path};

// Prints out content of the WASM module at the given path, as hexadecimal string.
// NOTE : Used for quick debugging purposes.
#[allow(unused)]
pub fn dumpModuleAsHexString(path: &str) {
  let mut file = File::open(Path::new(path)).unwrap();

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer).unwrap();

  println!(
    "WASM module contents as hexadecimal string : {}",
    getHexStringForBuffer(&buffer)
  );
}

pub fn getHexStringForBuffer(buffer: &Vec<u8>) -> String {
  return buffer.iter().map(|byte| format!("{:02x}", byte)).collect();
}
