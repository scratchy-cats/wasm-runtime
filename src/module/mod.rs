use std::{fs::File, path::Path};
use anyhow::Result;
use num_derive::FromPrimitive;
use reader::ModuleReader;
use sections::{FunctionSection, TypeSection};
use tracing::{info, instrument};

mod types;
mod indices;
mod sections;
mod reader;

#[derive(Default, FromPrimitive)]
pub enum BinaryVersion {

  #[default]
  One = 1
}

// WASM programs are organized into modules, which are the unit of deployment, loading, and
// compilation. A module collects definitions for types, functions, tables, memories, and globals.
// In addition, it can declare imports and exports and provide initialization in the form of data
// and element segments, or a start function.
#[derive(Default)]
pub struct Module {
  binaryVersion: BinaryVersion,

  typeSection:      Option<TypeSection>,
  functionSection:  Option<FunctionSection>
}

impl Module {
  // Constructs an instance of the Module struct by (buffered) reading the given WASM module. The
  // instance is then returned.
  #[instrument(skip(path), fields(module_path = path))]
  pub fn new(path: &str) -> Result<Self> {
    let file = File::open(Path::new(path))?;

    let mut moduleReader= ModuleReader::new(file);
    info!("reading WASM module at path {}", path);
    moduleReader.read( )
  }
}
