use anyhow::anyhow;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum NumType {
  I32 = 0x7F,
}

pub enum ValueType {
  Num(NumType),
}
impl TryFrom<u8> for ValueType {
  type Error = anyhow::Error;

  fn try_from(value: u8) -> std::prelude::v1::Result<Self, Self::Error> {
    if let Some(numType) = NumType::from_u8(value) {
      return Ok(ValueType::Num(numType));
    }

    Err(anyhow!("Invalid value type : {}", value))
  }
}

#[derive(Default)]
pub struct ResultType {
  pub valueTypes: Vec<ValueType>,
}

pub struct Expression {
  pub instructions: Vec<Instruction>,
}

pub struct FunctionSignature {
  pub inputs: ResultType,
  pub outputs: ResultType,
}

pub struct FunctionBody {
  pub locals: ResultType,
  pub body: Expression,
}

// Instructions are encoded by opcodes. Each opcode is represented by a single byte, and is
// followed by the instruction’s immediate arguments, where present.
// The only exceptions are structured control instructions, which consist of several opcodes
// bracketing their nested instruction sequences.
#[derive(PartialEq)]
pub enum Instruction {
  // Variable instructions.
  LocalGet(u32),

  // Numeric instructions.
  I32Add,

  End,
}

#[derive(FromPrimitive)]
pub enum Opcode {
  // Variable instructions.
  LocalGet = 0x20,

  // Numeric instructions.
  I32Add = 0x6A,

  End = 0x0B,
}
