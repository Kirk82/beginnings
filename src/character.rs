use rng::*;

//fields to put all data collected from the data file
#[derive(Default)]
pub struct Character {
    pub animal: Animal,
    pub number_of_legs: u32,
    pub animal_name: String,
    pub habitat: String,
    pub favourite_food: String,
}

//a list of possible animals
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, RandomVariant)]
pub enum Animal {
    #[default]
    Cat,
    Human,
    Dolphin,
}
