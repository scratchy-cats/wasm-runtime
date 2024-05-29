use anyhow::anyhow;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum NumType {
  I32 = 0x7F
}

pub enum ValueType {
  Num(NumType)
}
impl TryFrom<u8> for ValueType {
  type Error = anyhow::Error;

  fn try_from(value: u8) -> std::prelude::v1::Result<Self, Self::Error> {
    if let Some(numType)= NumType::from_u8(value) {
      return Ok(ValueType::Num(numType))}

    Err(anyhow!("Invalid value type : {}", value))
  }
}

#[derive(Default)]
pub struct ResultType {
  pub valueTypes: Vec<ValueType>
}

pub struct FunctionSignature {
  pub inputs: ResultType,
  pub outputs: ResultType
}
