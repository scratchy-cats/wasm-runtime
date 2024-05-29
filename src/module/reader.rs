use std::{fs::File, io::{self, BufReader, Cursor}};
use anyhow::{anyhow, Result};
use num_traits::FromPrimitive;
use tracing::{debug, instrument};
use crate::{module::{indices::TypeIndex, sections::{SectionId, TypeSection}, types::FunctionSignature}, utils::getHexStringForBuffer};
use super::{sections::FunctionSection, types::{ResultType, ValueType}, BinaryVersion, Module};

const MAGIC_STRING: &str= "\0asm";
const FUNCTION_SIGNATURE_STARTING_MARKER: u8= 0x60;

#[derive(Debug)]
pub struct ModuleReader<R>(R)
  where R: io::Read;

impl ModuleReader<BufReader<File>> {
  pub fn new(file: File) -> Self {
    // NOTE : Default buffer size is 8 KB.
    Self(BufReader::new(file))
  }

  // Constructs an instance of the Module struct by (buffered) reading the given WASM module.
  // The instance is then returned.
  pub(super) fn read(&mut self) -> Result<Module> {
    let mut module= Module::default( );

    /*
      The encoding of a module starts with a preamble containing :
        (1) a 4-byte magic string (\0asm).
        (2) a version field. The current version of the WASM binary format is 1.
    */
    module.binaryVersion= self.readPreamble( )?;

    /*
      The preamble is followed by a sequence of sections. Every section is optional - an omitted
      section is equivalent to the section being present with empty contents.

      Each section consists of :
        (1) a section id (1 byte).
        (2) the byte-size of the contents (as u32).
        (3) the actual contents, whose structure is dependent on the section id.
    */
    loop {
      let byte= self.readByte( )?;
      if byte == 0 {
        debug!("finished reading WASM module");
        break
      }

      let sectionIdAsU8= byte;
      let sectionId= SectionId::from_u8(sectionIdAsU8);
      if sectionId.is_none( ) {
        return Err(anyhow!("invalid section-id : {}", sectionIdAsU8))}
      let sectionId= sectionId.unwrap( );

      debug!("reading section : {}", sectionId);
      self.readSection(&mut module, sectionId)?;
      debug!("finished reading section");
    }

    Ok(module)
  }

  // Reads the preamble and validates the WASM binary version. Returns the WASM binary version.
  fn readPreamble(&mut self) -> Result<BinaryVersion> {
    debug!("reading magic string");
    let magicString= self.readString(4)?;
    if magicString != MAGIC_STRING {
      return Err(anyhow!("Magic string not found"))}

    debug!("reading binary version");
    let binaryVersionAsU32= u32::from_le_bytes(
      self.readBytes(4)?.as_slice( ).try_into( )?
    );
    if let Some(binaryVersion)= BinaryVersion::from_u32(binaryVersionAsU32) {
      debug!("binary version : {}", binaryVersionAsU32);
      return Ok(binaryVersion)
    }
    Err(anyhow!("wrong WASM binary version"))
  }

  #[instrument(skip(self, module))]
  fn readSection(&mut self, module: &mut Module, sectionId: SectionId) -> Result<( )> {
    let sectionContentSize= self.readU32( )?;
    debug!("content byte-size : {}", sectionContentSize);

    let sectionContent= self.readBytes(sectionContentSize as usize)?;
    let mut sectionContentReader= ModuleReader::<Cursor<Vec<u8>>>::new(sectionContent);

    debug!("reading content");
    match sectionId {
      SectionId::Type =>
        module.typeSection= Some(sectionContentReader.readTypeSectionContent( )?),

      SectionId::Function =>
        module.functionSection= Some(sectionContentReader.readFunctionSectionContent( )?)
    };
    debug!("finished reading content");

    Ok(( ))
  }
}

// Reading sections.
impl ModuleReader<Cursor<Vec<u8>>> {
  fn new(buffer: Vec<u8>) -> Self {
    ModuleReader(Cursor::new(buffer))
  }

  // All function types used in a module are defined in the type section.
  fn readTypeSectionContent(&mut self) -> Result<TypeSection> {
    let functionSignatureCount= self.readU32( )? as usize;
    debug!("function signatures count : {}", functionSignatureCount);

    let mut typeSection= TypeSection {
      functionSignatures: Vec::with_capacity(functionSignatureCount)
    };

    loop {
      let byte= self.readByte( )?;
      if byte == 0 { break }

      debug!("reading function signature");

      let functionSignatureStartingMarker= byte;
      if functionSignatureStartingMarker != FUNCTION_SIGNATURE_STARTING_MARKER {
        return Err(anyhow!("expected function signature starting marker, but found : {}", byte))}
      debug!("read function signature starting marker");

      let inputCount= self.readU32( )?;
      debug!("function input count : {}", inputCount);
      let inputs= self.readResultType(inputCount)?;

      let outputCount= self.readU32( )?;
      debug!("function output count : {}", outputCount);
      let outputs= self.readResultType(outputCount)?;

      debug!("finished reading function signature");
      typeSection.functionSignatures.push(FunctionSignature {
        inputs, outputs
      });
    }
    Ok(typeSection)
  }

  // All functions are defined in the function section.
  /*
    The  of a function declares its signature by reference to a type defined in the module. The
    parameters of the function are referenced through 0-based local indices in the function’s body.
    The parameters are mutable.

    The locals declare a vector of mutable local variables and their types. These variables are
    referenced through local indices in the function’s body. The index of the first local is the
    smallest index not referencing a parameter.

    The body is an instruction sequence that upon termination must produce a stack matching the
    function type’s result type.

    Functions are referenced through function indices, starting with the smallest index not
    referencing a function import.
  */
  fn readFunctionSectionContent(&mut self) -> Result<FunctionSection> {
    let functionCount= self.readU32( )? as usize;
    debug!("function count : {}", functionCount);

    let mut functionSection= FunctionSection {
      functions: Vec::with_capacity(functionCount)
    };

    loop {
      let result= self.readU32( );
      if result.as_ref( )
        .is_err_and(|error| error.to_string( ) == "failed to fill whole buffer") { break }

      debug!("reading function");

      let functionIndex= result.unwrap( );
      debug!("function index : {}", functionIndex);

      debug!("finished reading function");
      functionSection.functions.push(TypeIndex::Function(functionIndex));
    }
    Ok(functionSection)
  }
}

// Reading types.
impl<R> ModuleReader<R>
  where R: io::Read
{
  fn readResultType(&mut self, elementCount: u32) -> Result<ResultType> {
    let mut resultType= ResultType::default( );
    for _ in 0..elementCount {
      resultType.valueTypes.push(self.readValueType( )?);}
    Ok(resultType)
  }

  fn readValueType(&mut self) -> Result<ValueType> {
    ValueType::try_from(self.readByte( )?)
  }
}

// Utilities.
impl<R> ModuleReader<R>
  where R: io::Read
{
  fn readBytes(&mut self, byteCount: usize) -> Result<Vec<u8>> {
    let mut buffer= vec![0u8; byteCount];

    self.0.read(&mut buffer)?;
    debug!("read {} bytes : {}", byteCount, getHexStringForBuffer(&buffer));

    Ok(buffer)
  }

  fn readByte(&mut self) -> Result<u8> {
    Ok(self.readBytes(1)?[0])}

  fn readString(&mut self, len: usize) -> Result<String> {
    Ok(String::from_utf8_lossy(
      self.readBytes(len)?.as_slice( )
    ).to_string( ))
  }

  fn readU32(&mut self) -> Result<u32> {
    let number= leb128::read::unsigned(&mut self.0)?;
    Ok(u32::try_from(number)?)
  }
}
