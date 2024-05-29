use enum_display::EnumDisplay;
use num_derive::FromPrimitive;
use super::{indices::TypeIndex, types::FunctionSignature};

#[derive(FromPrimitive, EnumDisplay, Debug)]
pub enum SectionId {
  Type      = 0x01,
  Function  = 0x03
}

pub struct TypeSection {
  pub functionSignatures: Vec<FunctionSignature>
}

pub struct FunctionSection {
  pub functions: Vec<TypeIndex>
}

pub struct CodeSection { }
