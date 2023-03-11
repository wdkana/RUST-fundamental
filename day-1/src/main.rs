use std::io;
struct Person {
    age: i32,
}

fn calculate_age(person: Person, age: i32) {
    //* */ validating person age
    match person {
        Person { age: 0..=3 } => println!("usia anda {} dan anda adalah balita 🤓", age),
        Person { age: 4..=10 } => println!("usia anda {} dan anda adalah anak-anak 😋", age),
        Person { age: 11..=18 } => println!("usia anda {} dan anda adalah remaja 😎", age),
        Person { age: 19..=25 } => println!("usia anda {} dan anda adalah dewasa 😑", age),
        _ => println!("usia anda {} dan anda adalah orang tua 😇", age),
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
