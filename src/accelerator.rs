use drivers::Driver;
use motions::Motion;

pub struct Accelerator<D: Driver> {
    motions: Vec<Motion>,
    driver: D,
}

impl <D: Driver> Accelerator<D> {
    pub fn new(mots: Vec<Motion>, drv: D) -> Self {
        Accelerator{motions: mots, driver: drv}
    }

    fn within_bounds(&self, n: isize) -> isize{
        let min = 0;
        let max = self.motions.len() as isize;
        if n < min { return min }
        if n > max { return max }
        n
    }

    fn execute(&mut self, mut start: isize, mut finish: isize) -> &mut Self{
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
        self
    }
    pub fn shift(&mut self, n: isize) -> &mut Self{
        let start = self.driver.get_status();
        let finish = self.within_bounds(start + n);
        self.execute(start, finish);
        self
    }

    pub fn goto(&mut self, finish: isize) -> &mut Self{
        let status = self.driver.get_status();
        self.execute(status, finish);
        self
    }

    pub fn redo(&mut self) -> &mut Self{
        self.shift(-1);
        self.shift(1);
        self
    }

    pub fn up(&mut self) -> &mut Self{
        let last = self.motions.len() as isize;
        self.goto(last);
        self
    }

    pub fn down(&mut self) -> &mut Self{
        self.goto(0);
        self
    }

    pub fn reset(&mut self) -> &mut Self{
        let status = self.driver.get_status();
        self.execute(status,0);
        self.execute(0,status);
        self
    }

    pub fn get_status(&self) -> isize{
        return self.driver.get_status();
    }
}

pub enum Operation{
    Add(isize),
    Sub(isize)
}

impl Operation{
    fn get(finish: isize, start: isize) -> Self{
        if finish < start { return Operation::Sub(-1) }
        Operation::Add(1)
    }
}
