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

    //randomly selects a animal from the Animal enum using rng crate random enum value generator
    character.animal = Animal::random_variant(rng_generator);

    //initialises the number_of_legs varibale
    let _number_of_legs;

    //Says how many legs an character.animal has
    _number_of_legs = match character.animal {
        Animal::Cat => 4,
        Animal::Human => 2,
        Animal::Dolphin => 0,
    };

    //putting the correct number of legs into the character file based on which character.animal is randomly choosen
    character.number_of_legs = *data.number_of_legs_map.get(&character.animal).unwrap();

    //connects chosen character.animal and the appropriate habitats
    let potential_habitat = data.habitat_map.get(&character.animal).unwrap();

    //randomly chooses a habitat
    character.habitat = potential_habitat.sample(rng_generator).unwrap().to_string();

    //connects chosen character.animal and the appropriate names
    let potential_animal_names = data.animal_name_map.get(&character.animal).unwrap();

    //randomly selects a name and puts it in the character file
    character.animal_name = potential_animal_names
        .sample(rng_generator)
        .unwrap()
        .to_string();

    //assigning the appropriate list of favourite food base on which character.animal name is selected
    let potential_favourite_food = data.favourite_food_map.get(&character.animal_name).unwrap();

    //randomly choosing a favourite food from the selected list
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
        character.favourite_food,
    );
}
