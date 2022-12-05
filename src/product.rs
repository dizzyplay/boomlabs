use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
}

impl Product {
    pub fn new(id: u32, name: String) -> Self {
        Product { id, name }
    }
}

impl PrintInfo for Product {
    fn print_info(&self) {
        println!("product id {} name {}", self.id, self.name);
    }
}
