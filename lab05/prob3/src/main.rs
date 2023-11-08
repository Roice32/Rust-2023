use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    //phone: Option<String>,
    age: u8,
}

fn main() -> Result<()> {
    let input: String = fs::read_to_string("src/students_info.txt")?;
    let mut oldest_student: Student = Student {
        name: String::from(""),
        //phone: None,
        age: 0,
    };
    let mut youngest_student: Student = Student {
        name: String::from(""),
        //phone: None,
        age: 255,
    };

    for line in input.lines() {
        let current_student: Student = serde_json::from_str(&line)?;

        if current_student.age > oldest_student.age {
            oldest_student = current_student;
        } else if current_student.age < youngest_student.age {
            youngest_student = current_student;
        }
    }
    println!("Oldest student: {}", oldest_student.name);
    println!("Youngest student: {}", youngest_student.name);
    Ok(())
}
