use std::fmt::{self, Display};

fn main() {
    let e: Employee = Employee::new("John".to_string(), 30, "HR".to_string());
    println!("{}", e);
    println!("{:?}", e);
    println!("{:#?}", e);
}

struct Employee {
    name: String,
    age: i32,
    dept: String,
}

impl Employee {
    pub fn new(name: String, age: i32, dept: String) -> Self {
        Self { name, age, dept }
    }
}

impl fmt::Debug for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Employee")
            .field("Name", &self.name)
            .field("Age", &self.age)
            .field("Department", &self.dept)
            .finish()
    }
}

impl Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Employee [Name: {}, Age: {}, Department: {}]",
            self.name, self.age, self.dept
        )
    }
}