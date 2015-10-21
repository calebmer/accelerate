fn main() {
    println!("Hello, world!");
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
