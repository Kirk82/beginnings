#[derive(Default)]
pub struct Character {
    pub animal: Animal,
    pub number_of_legs: u32,
    pub animal_name: String,
    pub habitat: String,
    //pub favourite_food: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub enum Animal {
    #[default]
    Cat,
    Human,
    Dolphin,
}
