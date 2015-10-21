mod accelerator;
mod drivers;

use accelerator::Accelerator;
use drivers::Driver;
use drivers::TestDriver;

fn main() {
    // Test code
    let mut acc = Accelerator::new(vec![Motion{add: "add 1", sub:"sub 1"},
                                        Motion{add: "add 2", sub:"sub 2"},
                                        Motion{add: "add 3", sub:"sub 3"},
                                        Motion{add: "add 4", sub:"sub 4"}],
                                    TestDriver::new("Test Driver!"));
    println!("within_bounds(5) = {}", acc.within_bounds(5));
    println!("within_bounds(1) = {}", acc.within_bounds(1));
    acc.up();
    acc.down();
    acc.add();
    acc.add();
    acc.add();
    acc.sub();
    acc.redo();
    acc.reset();
}

pub enum Operation{
    Add(isize),
    Sub(isize)
}

impl Operation{
    fn get(finish: isize, start: isize) -> Self{
        println!("called OP get with \'{0}\' and \'{1}\'", finish,start);
        if finish < start { return Operation::Sub(-1) }
        Operation::Add(1)
    }
}

pub struct Motion{
    add: &'static str,
    sub: &'static str,
}
