use num_derive::FromPrimitive;
use super::types::FunctionSignature;

#[derive(FromPrimitive)]
pub enum SectionId {
  Type = 0x01
}

pub struct TypeSection {
  pub functionSignatures: Vec<FunctionSignature>
}
