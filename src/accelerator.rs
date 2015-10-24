use drivers::Driver;
use Motion;
use Operation;

pub struct Accelerator<D: Driver> {
    motions: Vec<Motion>,
    driver: D,
}

impl <D: Driver> Accelerator<D> {
    pub fn new(mots: Vec<Motion>, drv: D) -> Self {
        Accelerator{motions: mots, driver: drv}
    }
    // Needs to be un pub.ed
    pub fn within_bounds(&self, n: isize) -> isize{
        let min = 0;
        let max = self.motions.len() as isize;
        if n < min { return min }
        if n > max { return max }
        n
    }

    fn execute(&mut self, mut start: isize, mut finish: isize){
        if start != finish {
            start = self.within_bounds(start);
            finish = self.within_bounds(finish);
            let operation = Operation::get(finish, start);
            match operation{
                Operation::Add(op) => {
                    let mut i = start;
                    loop{
                        if i == finish { break }
                        self.driver.execute(self.motions[i as usize].add);
                        i += op;
                    }
                    self.driver.set_status(i);
                },
                Operation::Sub(op) => {
                    let mut i = start;
                    loop{
                        if i == finish { break }
                        i += op;
                        self.driver.execute(self.motions[i as usize].sub);
                    }
                    self.driver.set_status(i);
                }
            }
        }
    }
    pub fn go(&mut self, n: isize){
        let start = self.driver.get_status();
        let finish = self.within_bounds(start + n);
        self.execute(start,finish);
    }

    pub fn goto(&mut self, finish: isize){
        let status = self.driver.get_status();
        self.execute(status, finish);
    }

    pub fn add(&mut self){
        self.go(1);
    }

    pub fn sub(&mut self){
        self.go(-1);
    }

    pub fn redo(&mut self){
        self.sub();
        self.add();
    }

    pub fn up(&mut self){
        let last = self.motions.len() as isize;
        self.goto(last);
    }

    pub fn down(&mut self){
        self.goto(0);
    }

    pub fn reset(&mut self){
        let status = self.driver.get_status();
        self.execute(status,0);
        self.execute(0,status);
    }
}
