fn main() {
    let mark = 5;
    let student = Student::new(mark, 53, "Janusz".to_string());

    println!("{:#?}", student);

    let teacher = Teacher::new("Maths".to_string(), 14, "Boghdan".to_string());

    println!("{:#?}", teacher);
}

#[derive(Debug)]
struct Student {
    personal_data: PersonalData,
    mark: u8,
}

impl Student {
    fn new(mark: u8, age: u16, name: String) -> Self {
        Self { mark, personal_data: PersonalData::new(name, age) }
    }
}

#[derive(Debug)]
struct Teacher {
    personal_data: PersonalData,
    subject: String,
}

impl Teacher {
    fn new(subject: String, age: u16, name: String) -> Self {
        Self { subject, personal_data: PersonalData::new(name, age) }
    }
}

#[derive(Debug)]
struct PersonalData {
    name: String,
    age: u16,
}

impl PersonalData {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }
}
