use crate::*;

#[derive(Default)]
pub struct Character {
    pub animal: Animal,
    pub number_of_legs: i32,
    pub animal_name: String,
    pub habitat: String,
    pub favourite_food: String,
}

impl Character {
    pub fn new() -> Self {}
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
enum Animal {
    #[default]
    Cat,
    Human,
    Dolphin,
}
