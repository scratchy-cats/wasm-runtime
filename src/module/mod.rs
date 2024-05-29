use std::{fs::File, path::Path};
use anyhow::Result;
use num_derive::FromPrimitive;
use reader::ModuleReader;
use sections::TypeSection;

mod types;
mod sections;
mod reader;

#[derive(Default, FromPrimitive)]
pub enum BinaryVersion {

  #[default]
  One = 1
}

// The distributable, loadable, and executable unit of code in WebAssembly is called a module. A
// module collects definitions for types, functions, tables, memories, and globals. In addition, it
// can declare imports and exports and provide initialization.
#[derive(Default)]
pub struct Module {
  binaryVersion: BinaryVersion,

  typeSection: Option<TypeSection>
}

impl Module {
  // Constructs an instance of the Module struct by (buffered) reading the given WASM module. The
  // instance is then returned.
  pub fn new(path: &str) -> Result<Self> {
    let file = File::open(Path::new(path))?;

    let mut moduleReader= ModuleReader::new(file);
    moduleReader.read( )
  }
}
