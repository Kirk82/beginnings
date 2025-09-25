mod character;
mod data;

use character::*;
use data::*;

use std::collections::HashMap;

use rng::*;

fn main() {
    let rng_generator = Rng::new();

    let mut character = Character::default();
    let data = Data::new();

    //picks a random number
    let animal_selector = rng_generator.gen_range(1..4);

    // Assigns the random number to an animal from the Animal Enum
    let animal = match animal_selector {
        1 => Animal::Cat,
        2 => Animal::Human,
        3 => Animal::Dolphin,
        _ => panic!(),
    };

    character.number_of_legs = *data.number_of_legs_map.get(&animal).unwrap();

    //initialises the number_of_legs varibale
    let number_of_legs;

    //Says how many legs an animal has
    number_of_legs = match animal {
        Animal::Cat => 4,
        Animal::Human => 2,
        Animal::Dolphin => 0,
    };

    //connects the animal to the chosen animal variable
    let potential_habitat = data.habitat_map.get(&animal).unwrap();

    //randomly chooses a habitat
    character.habitat = potential_habitat.sample(rng_generator).unwrap().to_string();

    let potential_animal_names = data.animal_name_map.get(&animal).unwrap();

    character.animal_name = potential_animal_names
        .sample(rng_generator)
        .unwrap()
        .to_string();

    let potential_favourite_food = data.favourite_food_map.get(&animal).unwrap();

    character.favourite_food = potential_favourite_food
        .sample(rng_generator)
        .unwrap()
        .to_string();

    //Prints a line in terimal using randomly generated info from the data file and puts it into character file
    println!(
        "This {:?} has {} legs, lives in {} and is called {}. Their favourite food is {}",
        character.animal,
        character.number_of_legs,
        character.habitat,
        character.animal_name,
        character.favourite_food
    );
}
