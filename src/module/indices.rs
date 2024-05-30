/*
  Indices and index space :

  Definitions are referenced with zero-based indices. Each class of definition (section) has its
  own index space.

  The index space for functions, tables, memories and globals includes respective imports
  declared in the same module. The indices of these imports precede the indices of other
  definitions in the same index space.
  The index space for locals is only accessible inside a function and includes the parameters of
  that function, which precede the local variables.

  NOTE :

    (1) Element indices reference element segments and data indices reference data segments.

    (2) Label indices reference structured control instructions inside an instruction sequence.
*/

pub enum TypeIndex {
  Function(u32),
}
