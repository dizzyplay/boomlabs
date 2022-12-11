mod employee;
mod printinfo;
mod product;

use employee::{Employee, EmployeeRecords};
use printinfo::{dynamic_dispatch, static_dispatch};
use product::Product;

fn main() {
    let john = Employee::new(101, "John".to_string());
    let jane = Employee::new(102, "Jane".to_string());
    let tom = Employee::new(103, "Tom".to_string());

    let mut employee_records = EmployeeRecords::new();
    employee_records.push(john);
    EmployeeRecords::push(&mut employee_records, jane);
    employee_records.push(tom);

    println!("{:#?}", employee_records);
    println!(":Get 101: {:?}", employee_records.get(101));
    println!(":Get 102: {:?}", employee_records.get(102));

    println!("{:#?}", employee_records);
    println!(" next: {:?}", employee_records.next());
    println!("{:#?}", employee_records);
    println!(":Get 100: {:?}", employee_records.get(100));

    for em in &mut employee_records {
        println!("For {:#?}", em);
    }
    println!("{:#?}", employee_records);
}
