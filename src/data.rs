use crate::*;

pub struct Data {
    pub animal: Animal,
    pub number_of_animal_legs: i32,
    pub animal_name_map: HashMap<Animal, Vec<&'static str>>,
    pub habitat_map: HashMap<Animal, Vec<&'static str>>,
    pub favourite_food_map: HashMap<&'static str, Vec<&'static str>>,
}

impl Data {
    pub fn new() -> Self {
        let mut habitat_map: HashMap<Animal, Vec<&'static str>> = HashMap::new();

        habitat_map.insert(Animal::Cat, vec!["a house", "the wild"]);
        habitat_map.insert(Animal::Human, vec!["a house"]);
        habitat_map.insert(Animal::Dolphin, vec!["the Ocean", "Sea World"]);

        let mut animal_name_map = HashMap::new();

        animal_name_map.insert(Animal::Cat, vec!["Smudge", "Panther", "Whiskers"]);
        animal_name_map.insert(Animal::Dolphin, vec!["Flipper", "Speedy", "Mist"]);
        animal_name_map.insert(Animal::Human, vec!["James", "Andrew", "Peter"]);

        let mut favourite_food_map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        favourite_food_map.insert("Smudge", vec!["chicken", "treats"]);
        favourite_food_map.insert("Panther", vec!["Humans", "Dear"]);
        favourite_food_map.insert("Whiskers", vec!["cat food", "fish"]);
        favourite_food_map.insert("Flipper", vec!["Fish", "Planktin"]);
        favourite_food_map.insert("Speedy", vec!["Carp", "Sea trash"]);
        favourite_food_map.insert("Mist", vec!["Blood", "Clouds"]);
        favourite_food_map.insert("James", vec!["Burgers", "Potato cakes"]);
        favourite_food_map.insert("Andrew", vec!["Salad", "Tofu"]);
        favourite_food_map.insert("Peter", vec!["Kebabs", "Pizza"]);
    }
}
