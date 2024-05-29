use std::{fs::File, io::{self, BufReader, Cursor}};
use anyhow::{anyhow, Result};
use num_traits::FromPrimitive;
use crate::module::{sections::{SectionId, TypeSection}, types::FunctionSignature};
use super::{types::{ResultType, ValueType}, BinaryVersion, Module};

const MAGIC_STRING: &str= "\0asm";
const FUNCTION_SIGNATURE_STARTING_MARKER: u8= 0x60;

pub struct ModuleReader<R>(R)
  where R: io::Read;

impl ModuleReader<BufReader<File>> {
  pub fn new(file: File) -> Self {
    // NOTE : Default buffer size is 8 KB.
    Self(BufReader::new(file))
  }

  // Constructs an instance of the Module struct by (buffered) reading the given WASM module.
  // The instance is then returned.
  pub fn read(&mut self) -> Result<Module> {
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
      let sectionIdAsU8= self.readByte( )?;
      if sectionIdAsU8 == 0 { break }

      let sectionId= SectionId::from_u8(sectionIdAsU8);
      if sectionId.is_none( ) {
        return Err(anyhow!("Invalid section-id : {}", sectionIdAsU8))}
      println!("reading section with id : {}", sectionIdAsU8);

      let sectionContentSize= self.readU32( )?;
      println!("section content byte-size : {}", sectionContentSize);

      let sectionContent= self.readBytes(sectionContentSize as usize)?;
      let mut sectionContentReader= ModuleReader::<Cursor<Vec<u8>>>::new(sectionContent);

      match sectionId.unwrap( ) {
        SectionId::Type =>
          module.typeSection= Some(sectionContentReader.readTypeSectionContent( )?)
      };
    }

    Ok(module)
  }

  // Reads the preamble and validates the WASM binary version. Returns the WASM binary version.
  fn readPreamble(&mut self) -> Result<BinaryVersion> {
    let magicString= self.readString(4)?;
    if magicString != MAGIC_STRING {
      return Err(anyhow!("Magic string not found"))}

    let binaryVersion= u32::from_le_bytes(
      self.readBytes(4)?.as_slice( ).try_into( )?
    );
    if let Some(binaryVersion)= BinaryVersion::from_u32(binaryVersion) {
      return Ok(binaryVersion)}
    Err(anyhow!("Wrong WASM binary version"))
  }
}

// Reading sections.
impl ModuleReader<Cursor<Vec<u8>>> {
  fn new(buffer: Vec<u8>) -> Self {
    ModuleReader(Cursor::new(buffer))
  }

  fn readTypeSectionContent(&mut self) -> Result<TypeSection> {
    println!("reading type section");

    let functionSignaturesCount= self.readU32( )? as usize;
    println!("function signatures count in type section : {}", functionSignaturesCount);

    let mut typeSection= TypeSection {
      functionSignatures: Vec::new( )
    };

    loop {
      let byte= self.readByte( )?;
      if byte == 0 {
        return Ok(typeSection)}

      else if byte != FUNCTION_SIGNATURE_STARTING_MARKER {
        return Err(anyhow!("Expected function signature starting marker, but found : {}", byte))}
      

      let inputCount= self.readU32( )?;
      let inputs= self.readResultType(inputCount)?;

      let outputCount= self.readU32( )?;
      let outputs= self.readResultType(outputCount)?;

      typeSection.functionSignatures.push(FunctionSignature {
        inputs, outputs
      });
    }
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
