mod character;
mod data;

use character::*;
use data::*;

use std::collections::HashMap;

use rng::*;

fn main() {
    let rng_generator = Rng::new();

    let character = Character::new();
    let data = Data::new();

let character.number_of_legs = data.number_of_legs_map.get(animal).unwrap();

    //picks a random number
    let animal_selector = rng_generator.gen_range(1..4);

    // Assigns the random number to an animal from the Animal Enum
    let choosen_animal = match animal_selector {
        1 => Animal::Cat,
        2 => Animal::Human,
        3 => Animal::Dolphin,
        _ => panic!(),
    };

    //initialises the number_of_legs varibale
    let number_of_legs;

    //Says how many legs an animal has
    number_of_legs = match choosen_animal {
        Animal::Cat => 4,
        Animal::Human => 2,
        Animal::Dolphin => 0,
    };

    //This hashmap assigns different habitats to respective animals
    let mut habitat_map = HashMap::new();

    habitat_map.insert(Animal::Cat, vec!["a house", "the wild"]);
    habitat_map.insert(Animal::Human, vec!["a house"]);
    habitat_map.insert(Animal::Dolphin, vec!["the Ocean", "Sea World"]);

    //connects the animal to the chosen animal variable
    let potential_habitat = habitat_map.get(&choosen_animal).unwrap();

    //randomly chooses a habitat
    let habitat = potential_habitat.sample(rng_generator).unwrap();

    let mut animal_name_map = HashMap::new();

    animal_name_map.insert(Animal::Cat, vec!["Smudge", "Panther", "Whiskers"]);
    animal_name_map.insert(Animal::Dolphin, vec!["Flipper", "Speedy", "Mist"]);
    animal_name_map.insert(Animal::Human, vec!["James", "Andrew", "Peter"]);

    let potential_animal_names = animal_name_map.get(&choosen_animal).unwrap();

    let animal_name = potential_animal_names.sample(rng_generator).unwrap();

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

    let potential_favourite_food = favourite_food_map.get(animal_name).unwrap();

    let favourite_food = potential_favourite_food.sample(rng_generator).unwrap();

    //Prints a line in terimal using randomly generated info
    println!(
        "This {:?} has {} legs, lives in {} and is called {}. Their favourite food is {}",
        choosen_animal, number_of_legs, habitat, animal_name, favourite_food
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
enum Animal {
    #[default]
    Cat,
    Human,
    Dolphin,
}
