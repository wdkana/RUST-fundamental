use std::io;
struct Person {
    age: i32,
}

fn result(age: i32, age_type: &str, option: &str) {
    println!("usia anda {} dan anda adalah {} {}", age, age_type, option)
}

fn calculate_age(person: Person, age: i32) {
    //* */ validating person age
    match person {
        Person { age: 0..=3 } => result(age, "balita", "🤓"),
        Person { age: 4..=10 } => result(age, "anak-anak", "😋"),
        Person { age: 11..=18 } => result(age, "remaja", "😎"),
        Person { age: 19..=25 } => result(age, "dewasa", "😑"),
        _ => result(age, "orang tua", "😇"),
    }
}

fn main() {
    //? --lib input--
    let input = io::stdin();

    //? --input user--
    let mut age = String::new();
    println!("masukan usia anda: 👇");

    //? --get input age--
    input.read_line(&mut age).unwrap();

    //* */ --overide age string convert to int for calculation--
    let age: i32 = age.trim().parse().expect("need integer");

    //* */ --assign value to structure--
    let person = Person { age };

    //? --call function for the result--
    calculate_age(person, age);
}
