#[path = "./modules/utils.rs"]
mod utils;
use std::io;
fn main() {
    //? --lib input--
    let input = io::stdin();

    //? --input data--
    let mut name = String::new();
    println!("masukan nama anda: 👇");

    input.read_line(&mut name).unwrap();

    let mut age = String::new();
    println!("masukan usia anda: 👇");

    input.read_line(&mut age).unwrap();

    //* */ --overide age string convert to int for calculation--
    let age: i32 = age.trim().parse().expect("need integer");

    //? --validasi golongan berdasarkan usia--
    let age_type = utils::calculate_age(age);

    //* --isi struktur data person dari nilai input & hasil validasi-- */
    let person = Person {
        age: age,
        name: String::from(name),
        age_type: String::from(age_type),
    };

    //? --output--
    utils::console("halo", person.name);
    utils::console("usia kamu", person.age);
    utils::console("berarti kamu", person.age_type);
}

struct Person {
    age: i32,
    name: String,
    age_type: String,
}
