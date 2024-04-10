use std::collections::{HashMap, HashSet};
use std::option;

#[derive(Debug)]
pub enum Gender {
    FEMALE,
    MALE,
}

#[derive(Debug)]
pub struct Student {
    id: u32,
    name: String,
    age: u32,
    gender: Gender,
}

#[derive(Debug)]
pub struct Class {
    id: u32,
    name: String,
    students: HashSet<u32>, // store student ids
}


pub struct School {
    class: HashMap<u32, Class>,
    student: HashMap<u32, Student>,

    student_cnt: u32,
    class_cnt: u32,
}

impl School {
    pub fn new() -> School{
        School {
            class: HashMap::new(),
            student: HashMap::new(),
            student_cnt: 0,
            class_cnt: 0
        }
    }

    pub fn student_cnt(&self) -> u32 {
        self.student_cnt
    }

    pub fn class_cnt(&self) -> u32 {
        self.class_cnt
    }

    pub fn create_student(&mut self, name: String, age: u32, gender: Gender) -> u32 {
        self.student_cnt += 1;
        let id = self.student_cnt;

        let student = Student {
            id,
            name,
            age,
            gender,
        };

        self.student.insert(id, student);

        id
    }

    pub fn create_class(&mut self, name: String) -> u32 {
        self.class_cnt += 1;
        let id = self.class_cnt;

        let class = Class {
            id,
            name,
            students: HashSet::new(),
        };

        self.class.insert(id, class);

        id
    }

    pub fn add_student_to_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(class) = self.class.get_mut(&class_id) {
            class.students.insert(student_id);
        } else {
            println!("class not found, id: {}", class_id);
        }
    }

    pub fn remove_student_from_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(class) = self.class.get_mut(&class_id) {
            if class.students.remove(&student_id) {
                println!("remove student from class success, student_id:{} class_id:{}", student_id, class_id);
            } else {
                println!("student not in class, student_id:{} class_id:{}", student_id, class_id);
            }
        }
    }

    pub fn query_all_students_from_class(&self, class_id: u32) -> Vec<&Student> {
        let mut students = Vec::new();
        if let Some(class) = self.class.get(&class_id) {
            let student_ids = &class.students;
            for student_id in student_ids {
                if let Some(student) = self.student.get(&student_id) {
                    students.push(student);
                }
            }
        } else {
            println!("class not found, id: {}", class_id);
        }

        students
    }

    pub fn query_student_from_class(&self, class_id: u32, student_id: u32) -> Option<&Student> {
        let mut result = None;

        if let Some(class) = self.class.get(&class_id) {
            if let Some(_) = class.students.get(&student_id) {
                if let Some(student) = self.student.get(&student_id) {
                    result = Some(student);
                }
            } else {
                println!("student not found, id: {}", student_id);
            }
        } else {
            println!("class not found, id: {}", class_id);
        }

        result
    }
}