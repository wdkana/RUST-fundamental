use std::fmt::Display;

pub fn log<M: Display, V: Display>(message: M, var: V) {
    println!("{} {}", message, var);
}
