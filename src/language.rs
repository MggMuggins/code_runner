pub trait Language {
    fn run();
}


pub struct Python {}

impl Python {
    
    pub fn new(code: String) -> Python {
        Python {}
    }
}

impl Language for Python {
    fn run() {
        println!("I got run!");
    }
}
