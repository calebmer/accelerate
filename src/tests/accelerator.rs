#![allow(unused_must_use)]
use drivers::Driver;
use drivers::default::DefaultDriver as Test;
use motions::Motion;
use accelerator;

impl Motion {
  pub fn test(n: usize) -> Self {
    Motion {
      name: "test".to_string(),
      add: "add: ".to_string() + &n.to_string(),
      sub: "sub: ".to_string() + &n.to_string(),
      version: vec![n, n + 1, n + 2],
      extension: String::from(""),
    }
  }
}

fn get_motions() -> Vec<Motion> {
  (0..8).map(Motion::test).collect()
}

fn get_driver() -> Box<Test> {
  Box::new(Test::new("Test Driver".to_string()))
}

#[test]
fn up() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::up(&mut drv, &mots);
  assert_eq!(mots.len() as isize, drv.get_status().unwrap());
}

#[test]
fn down() {
  let mut drv = get_driver();
  accelerator::down(&mut drv, &get_motions());
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn down_up() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::down(&mut drv, &mots);
  accelerator::up(&mut drv, &mots);
  assert_eq!(mots.len() as isize, drv.get_status().unwrap());
}

#[test]
fn up_down() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::up(&mut drv, &mots);
  accelerator::down(&mut drv, &mots);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn redo() {
  let mut drv = get_driver();
  accelerator::redo(&mut drv, &get_motions());
  // sub at 0 will do nothing and then add thus the status should be 1
  assert_eq!(1, drv.get_status().unwrap());
}

#[test]
fn shift2_redo() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::shift(&mut drv, &mots, 2);
  accelerator::redo(&mut drv, &mots);
  assert_eq!(2, drv.get_status().unwrap());
}

#[test]
fn shift() {
  let mut drv = get_driver();
  accelerator::shift(&mut drv, &get_motions(), 0);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn shift_n4() {
  let mut drv = get_driver();
  accelerator::shift(&mut drv, &get_motions(), -4);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn shift_3() {
  let mut drv = get_driver();
  accelerator::shift(&mut drv, &get_motions(), 3);
  assert_eq!(3, drv.get_status().unwrap());
}

#[test]
fn shift_max_p2() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::shift(&mut drv, &mots, (mots.len() + 2) as isize);
  assert_eq!(mots.len() as isize, drv.get_status().unwrap());
}

#[test]
fn goto() {
  let mut drv = get_driver();
  accelerator::goto(&mut drv, &get_motions(), 0);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn goto_n5() {
  let mut drv = get_driver();
  accelerator::goto(&mut drv, &get_motions(), -5);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn goto_max_p2() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::goto(&mut drv, &mots, (mots.len() + 2) as isize);
  assert_eq!(mots.len() as isize, drv.get_status().unwrap());
}

#[test]
fn goto_3() {
  let mut drv = get_driver();
  accelerator::goto(&mut drv, &get_motions(), 3);
  assert_eq!(3, drv.get_status().unwrap());
}

#[test]
fn goto_6_n1() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::goto(&mut drv, &mots, 6);
  accelerator::goto(&mut drv, &mots, -1);
  assert_eq!(0, drv.get_status().unwrap());
}

#[test]
fn goto_2_reset() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::goto(&mut drv, &mots, 2);
  accelerator::reset(&mut drv, &mots);
  assert_eq!(2, drv.get_status().unwrap());
}

#[test]
fn goto_max_p2_reset() {
  let mut drv = get_driver();
  let mots = get_motions();
  accelerator::goto(&mut drv, &mots, (mots.len() + 2) as isize);
  accelerator::reset(&mut drv, &mots);
  assert_eq!(mots.len() as isize, drv.get_status().unwrap());
}

#[test]
fn reset() {
  let mut drv = get_driver();
  accelerator::reset(&mut drv, &get_motions());
  assert_eq!(0, drv.get_status().unwrap());
}
