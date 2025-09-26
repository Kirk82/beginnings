use crate::*;

pub struct Data {
    pub animal_name_map: HashMap<Animal, Vec<&'static str>>,
    pub habitat_map: HashMap<Animal, Vec<&'static str>>,
    //pub favourite_food_map: HashMap<String, Vec<String>>,
    pub number_of_legs_map: HashMap<Animal, u32>,
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

        //let mut favourite_food_map: HashMap<String, Vec<String>> = HashMap::new();
        //
        //favourite_food_map.insert(
        //    "Smudge".to_string(),
        //    vec!["chicken".to_string(), "treats".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Panther".to_string(),
        //    vec!["Humans".to_string(), "Dear".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Whiskers".to_string(),
        //    vec!["cat food".to_string(), "fish".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Flipper".to_string(),
        //    vec!["Fish".to_string(), "Planktin".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Speedy".to_string(),
        //    vec!["Carp".to_string(), "Sea trash".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Mist".to_string(),
        //    vec!["Blood".to_string(), "Clouds".to_string()],
        //);
        //favourite_food_map.insert(
        //    "James".to_string(),
        //    vec!["Burgers".to_string(), "Potato cakes".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Andrew".to_string(),
        //    vec!["Salad".to_string(), "Tofu".to_string()],
        //);
        //favourite_food_map.insert(
        //    "Peter".to_string(),
        //    vec!["Kebabs".to_string(), "Pizza".to_string()],
        //);

        let mut number_of_legs: HashMap<Animal, u32> = HashMap::new();

        number_of_legs.insert(Animal::Cat, 4);
        number_of_legs.insert(Animal::Dolphin, 0);
        number_of_legs.insert(Animal::Human, 2);

        return Self {
            animal_name_map: animal_name_map,
            habitat_map: habitat_map,
            //favourite_food_map: favourite_food_map,
            number_of_legs_map: number_of_legs,
        };
    }
}
