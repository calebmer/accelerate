use drivers::{Driver, DefaultDriver};
use motions::Motion;
use accelerator;

fn get_motions() -> Vec<Motion> {
    return vec![Motion::get("add 1".to_string(), "sub 1".to_string()),
                Motion::get("add 2".to_string(), "sub 2".to_string()),
                Motion::get("add 3".to_string(), "sub 3".to_string()),
                Motion::get("add 4".to_string(), "sub 4".to_string())];
}

fn get_driver() -> DefaultDriver {
    return DefaultDriver::new("Test Driver".to_string());
}

#[test]
fn up() {
    let mut drv = get_driver();
    accelerator::up(&mut drv, &get_motions());
    assert_eq!(4, drv.get_status());
}

#[test]
fn down() {
    let mut drv = get_driver();
    accelerator::down(&mut drv, &get_motions());
    assert_eq!(0, drv.get_status());
}

#[test]
fn down_up() {
    let mut drv = get_driver();
    accelerator::down(&mut drv, &get_motions());
    accelerator::up(&mut drv, &get_motions());
    assert_eq!(4, drv.get_status());
}

#[test]
fn up_down() {
    let mut drv = get_driver();
    accelerator::up(&mut drv, &get_motions());
    accelerator::down(&mut drv, &get_motions());
    assert_eq!(0, drv.get_status());
}

#[test]
fn redo() {
    let mut drv = get_driver();
    accelerator::redo(&mut drv, &get_motions());
    // sub at 0 will do nothing and then add thus the status should be 1
    assert_eq!(1, drv.get_status());
}

#[test]
fn shift2_redo() {
    let mut drv = get_driver();
    accelerator::shift(&mut drv, &get_motions(), 2);
    accelerator::redo(&mut drv, &get_motions());
    assert_eq!(2, drv.get_status());
}

#[test]
fn shift() {
    let mut drv = get_driver();
    accelerator::shift(&mut drv, &get_motions(), 0);
    assert_eq!(0, drv.get_status());
}

#[test]
fn shift_n4() {
    let mut drv = get_driver();
    accelerator::shift(&mut drv, &get_motions(), -4);
    assert_eq!(0, drv.get_status());
}

#[test]
fn shift_3() {
    let mut drv = get_driver();
    accelerator::shift(&mut drv, &get_motions(), 3);
    assert_eq!(3, drv.get_status());
}

#[test]
fn shift_8() {
    let mut drv = get_driver();
    accelerator::shift(&mut drv, &get_motions(), 8);
    assert_eq!(4, drv.get_status());
}

#[test]
fn goto() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 0);
    assert_eq!(0, drv.get_status());
}

#[test]
fn goto_n5() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), -5);
    assert_eq!(0, drv.get_status());
}

#[test]
fn goto_8() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 8);
    assert_eq!(4, drv.get_status());
}

#[test]
fn goto_3() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 3);
    assert_eq!(3, drv.get_status());
}

#[test]
fn goto_6_n1() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 6);
    accelerator::goto(&mut drv, &get_motions(), -1);
    assert_eq!(0, drv.get_status());
}

#[test]
fn goto_2_reset() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 2);
    accelerator::reset(&mut drv, &get_motions());
    assert_eq!(2, drv.get_status());
}

#[test]
fn goto_8_reset() {
    let mut drv = get_driver();
    accelerator::goto(&mut drv, &get_motions(), 8);
    accelerator::reset(&mut drv, &get_motions());
    assert_eq!(4, drv.get_status());
}

#[test]
fn reset() {
    let mut drv = get_driver();
    accelerator::reset(&mut drv, &get_motions());
    assert_eq!(0, drv.get_status());
}
