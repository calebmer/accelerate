/// Defines the two possible directions of a motion.
pub enum Operation {
  /// Traditionally the up direction in other migration software. Adds some
  /// things to the driver. Should be reversable with a sub operation.
  Add,
  /// Traditionally the down direction in other migration software. Should remove any
  /// changes made with the corresponding add operation.
  Sub,
}

impl Operation {
  /// Takes two numbers and returns the operation required to get there. If
  /// the first (finish) parameter is less we must be subtracting, if the
  /// first parameter is greater we must be adding.
  pub fn get(finish: isize, start: isize) -> Self {
    if finish < start {
      return Operation::Sub;
    }
    Operation::Add
  }

  /// The integer result represents the algebraic direction (positive).
  pub fn add() -> isize { 1 }

  /// The integer result represents the algebraic direction (negative).
  pub fn sub() -> isize { -1 }
}
