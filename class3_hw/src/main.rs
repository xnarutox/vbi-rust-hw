use std::collections::HashMap;

fn main() {
    let mut school = School::new();
    school.add(2, "Yamamoto");
    school.add(3, "KanaIe");
    school.add(5, "Suzuki");
    school.add(4, "Takahashi");
    school.add(5, "Yamada");
    school.add(5, "Sumimoto");

    let list_student_same_grade = school.grade(5);
    println!("{:?}", list_student_same_grade);
}

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School { students: HashMap::new() }

    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.parse().unwrap(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut list_grade = vec![];
        for (_name, grade) in self.students.iter() {
            list_grade.push(*grade);
        }
        list_grade.sort();
        list_grade.dedup();
        list_grade
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {

        let mut list_name = vec![];
        for (name, _grade ) in self.students.iter() {
            if *_grade == grade {
                list_name.push(name.to_string());
            }
        }
        list_name.sort();
        list_name
        
    }
}