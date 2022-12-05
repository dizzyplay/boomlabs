mod employee;
mod printinfo;
mod product;

use employee::Employee;
use printinfo::{dynamic_dispatch, static_dispatch};
use product::Product;

fn main() {
    let e = Employee::new("hello".to_string());
    let p = Product::new(1, "p".to_string());
    static_dispatch(e.clone());
    dynamic_dispatch(Box::new(e));

    static_dispatch(p.clone());
    dynamic_dispatch(Box::new(p));
}
