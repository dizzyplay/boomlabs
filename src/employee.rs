use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Employee {
    pub name: String,
}

impl Employee {
    pub fn new(name: String) -> Self {
        Employee { name }
    }
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(" name is {}", self.name);
    }
}
