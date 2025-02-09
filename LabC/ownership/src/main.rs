
struct Person {
    name: String,
    age: u32
}

impl Person {

    fn new(name_param: &str, age_param: u32) -> Person {
        Person {
            name: name_param.to_string(),
            age: age_param
        }
    }
}

fn main() {
    let mut p1 = Person::new("Jane", 30);

    let r1 = & p1;
    let r2 = & p1;

    print_person(r1);
    print_person(r2);

    let r3 = &mut p1;

    increment_age(r3);

    print_person(r3);

    increment_age(r3);

    print_person(&r3);
}

fn print_person(p: &Person) {
    println!("{} is {} years old", p.name, p.age);
}

fn increment_age(p: &mut Person) {
    p.age = p.age + 1;
}