use crate::printinfo::PrintInfo;

#[derive(Debug, Clone)]
pub struct Employee {
    pub id: usize,
    pub name: String,
}

impl Employee {
    pub fn new(id: usize, name: String) -> Self {
        Employee { name, id }
    }
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(" name is {}", self.name);
    }
}

#[derive(Debug, Clone)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}

impl EmployeeRecords {
    pub fn new() -> Self {
        Self {
            idx: 0,
            employees: Vec::new(),
        }
    }
    pub fn push(&mut self, employee: Employee) {
        self.employees.push(employee);
    }
    pub fn get(&self, id: usize) -> Option<&Employee> {
        self.employees.iter().find(|e| e.id == id)
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            return Some(output);
        } else {
            return None;
        }
    }
}
