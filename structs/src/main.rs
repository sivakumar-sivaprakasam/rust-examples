use std::fmt::{self, Display};

fn main() {
    let e: Employee = Employee::new("John".to_string(), 30, "HR".to_string());
    println!("{}", e);
    println!("{:?}", e);
    println!("{:#?}", e);

    let ts = TupleStruct("John".to_string(), 30, "HR".to_string());
    println!("{}", ts);
    println!("{:?}", ts);
    println!("{:#?}", ts);
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

struct TupleStruct(String, i32, String);

impl fmt::Debug for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Employee")
            .field("Name", &self.0)
            .field("Age", &self.1)
            .field("Department", &self.2)
            .finish()
    }
}

impl Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Employee [Name: {}, Age: {}, Department: {}]",
            self.0, self.1, self.2
        )
    }
}
