use anyhow::Result;
use std::fs;
use std::str::FromStr;

struct Student {
    name: String,
    phone: String,
    age: u8,
}

fn main() -> Result<()> {
    let input: String = fs::read_to_string("src/students_info.txt")?;
    let mut oldest_student: Student = Student {
        name: String::from(""),
        phone: String::from(""),
        age: 0,
    };
    let mut youngest_student: Student = Student {
        name: String::from(""),
        phone: String::from(""),
        age: 255,
    };

    for line in input.lines() {
        let mut i: u8 = 0;
        let mut current_student: Student = Student {
            name: String::from(""),
            phone: String::from(""),
            age: 0,
        };
        for field in line.split(',') {
            match i {
                0 => current_student.name = String::from(field),
                1 => current_student.phone = String::from(field),
                2 => current_student.age = u8::from_str(field)?,
                3..=u8::MAX => {}
            }
            i += 1;
        }
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
