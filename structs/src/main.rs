#![allow(dead_code)]

enum Status {
    Archived,
    InProgress,
    Completed,
}

struct Update {
    project_status:  Status,
    description: String,

}

struct Project {
    owner: u32, // identified by m#
    title: String,
    description: String,
    start_date: [u16; 3],
    updates: Vec<Update>,
}

struct Member {
    mustang_number: u32,
    first_name: String,
    last_name: String,
    start_date: [u16; 3],
    project: Project,
}

fn demonstrate_struct_instantiation(){
    let member: Member = Member {
        mustang_number: 20240140,
        first_name: String::from("Jamil"),
        last_name: String::from("Bousquet"),
        start_date: [2, 24, 2023],
        project: Project {
            owner: 0,
            title: String::from("Creating Simplectic Method Implementation"),
            description: String::from(""),
            start_date: [2, 27, 2023],
            updates: Vec::from([Update {
                project_status: Status::InProgress,
                description: String::from("Learning Rust")
            }])
        }
    };

    println!("Member's M#: {}", member.mustang_number);
}

fn main() {
    demonstrate_struct_instantiation();
}
