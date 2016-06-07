use error::Error;
use motions::Motion;

#[derive(Eq, PartialEq, Debug)]
struct State {
  applied: Vec<Motion>,
  unapplied: Vec<Motion>,
}

fn diff_motions(mut motion_names: Vec<String>, mut motions: Vec<Motion>) -> Result<State, Error> {
  // Make sure we never have more applied motions than we have expected motions.
  if !(motions.len() >= motion_names.len()) {
    return Err(Error::new(format!(
      "There are {} motions that have been applied which is more than the {} motions we know of.",
      motion_names.len(),
      motions.len()
    )));
  }

  // Create our state variable.
  let mut state = State {
    applied: Vec::new(),
    unapplied: Vec::new(),
  };

  // We have an index for error reporting purposes. The index starts at -1
  // because we increment when the loop starts so it will then become 0.
  let mut index = -1;

  // Reverse our arrays because we want to pop the first items not the last
  // ones.
  motion_names.reverse();
  motions.reverse();

  // Left the infinite loop, begin!
  loop {
    // Increment our index.
    index += 1;
    // Get the motion that was applied…
    if let Some(motion_name) = motion_names.pop() {
      // Get the motion that we actually expect…
      if let Some(motion) = motions.pop() {
        // If the motion we expected was the one that actually got applied—we
        // good, continue after saving this motion.
        if motion_name == motion.name {
          state.applied.push(motion);
        }
        // Otherwise, if the motions are different, something bad happened.
        // We should let the user know.
        else {
          return Err(Error::new(format!(
            "The '{}' motion we expected is not the same as the '{}' motion that was actually applied at index {}. Try manually applying this motion with `accelerate apply {}`.",
            motion.name,
            motion_name,
            index,
            motion.add_path.display()
          )));
        }
      }
      // Because of our check earlier, there should always be more expected
      // motions than applied motions.
      else {
        unreachable!();
      }
    }
    // If we have no more applied motions, return the rest of our expected
    // motions.
    else {
      // Reverse the motions back before appending them so we get the original
      // order.
      motions.reverse();
      state.unapplied.append(&mut motions);
      return Ok(state);
    }
  }
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};
  use motions::Motion;
  use super::{State, diff_motions};

  fn pb(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
  }

  fn motion_a() -> Motion {
    Motion {
      name: "a".to_string(),
      add_path: pb("a.add"),
      sub_path: pb("a.sub"),
    }
  }

  fn motion_b() -> Motion {
    Motion {
      name: "b".to_string(),
      add_path: pb("b.add"),
      sub_path: pb("b.sub"),
    }
  }

  fn motion_c() -> Motion {
    Motion {
      name: "c".to_string(),
      add_path: pb("c.add"),
      sub_path: pb("c.sub"),
    }
  }

  #[test]
  fn test_diff_motions_extra_names() {
    assert!(diff_motions(vec!["a".to_string(), "b".to_string()], vec![motion_a()]).is_err());
  }

  #[test]
  fn test_diff_motions_unequal() {
    assert!(diff_motions(vec!["c".to_string(), "a".to_string()], vec![motion_a(), motion_b(), motion_c()]).is_err());
  }

  #[test]
  fn test_diff_motions_all() {
    assert_eq!(diff_motions(
      vec!["a".to_string(), "b".to_string(), "c".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a(), motion_b(), motion_c()],
      unapplied: vec![],
    });
  }

  #[test]
  fn test_diff_motions_some() {
    assert_eq!(diff_motions(
      vec!["a".to_string(), "b".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a(), motion_b()],
      unapplied: vec![motion_c()],
    });
    assert_eq!(diff_motions(
      vec!["a".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a()],
      unapplied: vec![motion_b(), motion_c()],
    });
  }

  #[test]
  fn test_diff_motions_none() {
    assert_eq!(diff_motions(
      vec![],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![],
      unapplied: vec![motion_a(), motion_b(), motion_c()],
    });
  }
}
