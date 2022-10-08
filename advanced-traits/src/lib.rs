pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("***");
    }
}
