pub fn console<M: std::fmt::Display, V: std::fmt::Display>(message: M, var: V) {
    println!("{} {}", message, var)
}

pub fn calculate_age(age: i32) -> String {
    //* */ validating person age
    match age {
        0..=3 => return "balita 🤓".to_owned(),
        4..=10 => return "anak-anak 😋".to_owned(),
        11..=18 => return "remaja 😎".to_owned(),
        19..=25 => return "dewasa 😑".to_owned(),
        _ => return "orang tua 😇".to_owned(),
    }
}
