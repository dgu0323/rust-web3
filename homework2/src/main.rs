use std::collections::HashMap;
use crate::school::{Gender, School};

mod school;


fn main() {

    println!("start test");

    let mut school = School::new();

    let studentA = school.create_student(String::from("a"), 20, Gender::MALE);
    let studentB = school.create_student(String::from("b"), 30, Gender::FEMALE);
    let studentC = school.create_student(String::from("c"), 40, Gender::FEMALE);
    let studentD = school.create_student(String::from("c"), 40, Gender::FEMALE);

    let class1 = school.create_class(String::from("class1"));
    let class2 = school.create_class(String::from("class2"));

    school.add_student_to_class(studentA, class1);
    school.add_student_to_class(studentB, class1);
    school.add_student_to_class(studentC, class2);
    school.add_student_to_class(studentD, class2);

    let student_cnt = school.student_cnt();
    let class_cnt = school.class_cnt();

    println!("student_cnt: {}", student_cnt);
    println!("class_cnt: {}", class_cnt);

    let class1_students = school.query_all_students_from_class(class1);
    println!("class1's students: ");
    for s in class1_students {
        println!("{:?}", s)
    }


    let will_found_student = 33;
    if let Some(s) = school.query_student_from_class(class1, will_found_student) {
        println!("found student is {:?}", studentB);
    } else {
        println!("not found student");
    }


    school.remove_student_from_class(studentC, class2);
    println!("class2's students: ");
    let class2_students = school.query_all_students_from_class(class2);
    for s2 in class2_students {
        println!("{:?}", s2);
    }

    println!("start done");
}
