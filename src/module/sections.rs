use super::{
  indices::TypeIndex,
  types::{FunctionBody, FunctionSignature},
};
use enum_display::EnumDisplay;
use num_derive::FromPrimitive;

#[derive(FromPrimitive, EnumDisplay, Debug)]
pub enum SectionId {
  Type = 0x01,
  Function = 0x03,
	Start = 0x08,
  Code = 0x0a,
}

pub struct TypeSection {
  pub functionSignatures: Vec<FunctionSignature>,
}

pub struct FunctionSection {
  pub functions: Vec<TypeIndex>,
}

pub struct StartSection {}

pub struct CodeSection {
  pub functionBodies: Vec<FunctionBody>,
}
