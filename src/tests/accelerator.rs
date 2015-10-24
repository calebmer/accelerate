use drivers::Driver;
use accelerator::*;

struct TestDriver{
    target: &'static str,
    status: isize,
}

impl Driver for TestDriver {
    fn new(target: &'static str) -> Self {
        TestDriver { target: target, status: 0 }
    }

    fn get_status(&self) -> isize{
        println!("Get Status of {0}, which is \'{1}\'", self.target, self.status);
        return self.status
    }

    fn set_status(&mut self, status: isize){
        println!("Set Status to \'{}\'", status);
        self.status = status;
    }

    fn execute(&self, motion: &'static str){
        println!("I am a {1} that says \'{0}\'", motion, self.target);
    }
}

fn get_accelerator() -> Accelerator<TestDriver>{
    return Accelerator::new(vec![Motion{add: "add 1", sub:"sub 1"},
                                        Motion{add: "add 2", sub:"sub 2"},
                                        Motion{add: "add 3", sub:"sub 3"},
                                        Motion{add: "add 4", sub:"sub 4"}],
                     TestDriver::new("Test Driver!"));
}

#[test]
fn up() {
    let mut acc = get_accelerator();
    acc.up();
    assert_eq!(4, acc.get_status());
}

#[test]
fn down() {
    let mut acc = get_accelerator();
    acc.down();
    assert_eq!(0, acc.get_status());
}

#[test]
fn up_down() {
    let mut acc = get_accelerator();
    acc.up();
    acc.down();
    assert_eq!(0, acc.get_status());
}

#[test]
fn add() {
    let mut acc = get_accelerator();
    acc.add();
    assert_eq!(1, acc.get_status());
}

#[test]
fn sub() {
    let mut acc = get_accelerator();
    acc.sub();
    assert_eq!(0, acc.get_status());
}

#[test]
fn add_sub() {
    let mut acc = get_accelerator();
    acc.add();
    acc.sub();
    assert_eq!(0, acc.get_status());
}

#[test]
fn redo() {
    let mut acc = get_accelerator();
    acc.redo();
    // sub at 0 will do nothing and then add thus the status should be 1
    assert_eq!(1, acc.get_status());
}

#[test]
fn add2_redo() {
    let mut acc = get_accelerator();
    acc.add();
    acc.add();
    acc.redo();
    assert_eq!(2, acc.get_status());
}
#[test]
fn go() {
    let mut acc = get_accelerator();
    acc.go(0);
    assert_eq!(0, acc.get_status());
}

#[test]
fn go_2() {
    let mut acc = get_accelerator();
    acc.go(2);
    assert_eq!(2, acc.get_status());
}

#[test]
fn go_8() {
    let mut acc = get_accelerator();
    acc.go(8);
    assert_eq!(4, acc.get_status());
}

#[test]
fn goto() {
    let mut acc = get_accelerator();
    acc.goto(0);
    assert_eq!(0, acc.get_status());
}

#[test]
fn goto_8() {
    let mut acc = get_accelerator();
    acc.goto(8);
    assert_eq!(4, acc.get_status());
}

#[test]
fn goto_3() {
    let mut acc = get_accelerator();
    acc.goto(3);
    assert_eq!(3, acc.get_status());
}

#[test]
fn goto_6_n1() {
    let mut acc = get_accelerator();
    acc.goto(6);
    acc.goto(-1);
    assert_eq!(0, acc.get_status());
}

#[test]
fn reset() {
    let mut acc = get_accelerator();
    acc.reset();
    assert_eq!(0, acc.get_status());
}

#[test]
fn goto_2_reset() {
    let mut acc = get_accelerator();
    acc.goto(2);
    acc.reset();
    assert_eq!(2, acc.get_status());
}

#[test]
fn goto_8_reset() {
    let mut acc = get_accelerator();
    acc.goto(8);
    acc.reset();
    assert_eq!(4, acc.get_status());
}
