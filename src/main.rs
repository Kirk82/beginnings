mod character;
mod data;

use character::*;
use data::*;

pub use rng::*;
use std::collections::HashMap;

fn main() {
    let rng = Rng::new();

    let mut character = Character::default();
    let data = Data::new();

    //randomly selects a animal from the Animal enum using rng crate random enum value generator
    character.animal = Animal::random_variant(rng);

    //putting the correct number of legs into the character struct based on which character.animal is randomly choosen
    character.number_of_legs = *data.number_of_legs_map.get(&character.animal).unwrap();

    //connects chosen character.animal and the appropriate habitats
    let potential_habitats = data.habitat_map.get(&character.animal).unwrap();

    //randomly chooses a habitat
    character.habitat = potential_habitats.sample(rng).unwrap().to_string();

    //connects chosen character.animal and the appropriate names
    let potential_animal_names = data.animal_name_map.get(&character.animal).unwrap();

    //randomly selects a name and puts it in the character struct
    character.animal_name = potential_animal_names.sample(rng).unwrap().to_string();

    //assigning the appropriate list of favourite food based on selected character.animal_name
    let potential_favourite_foods = data.favourite_food_map.get(&character.animal_name).unwrap();

    //randomly choosing a favourite food from the selected list
    character.favourite_food = potential_favourite_foods.sample(rng).unwrap().to_string();

    //Reads the character and prints output to the terminal
    println!(
        "This {:?} has {} legs, lives in {} and is called {}. Their favourite food is {}.",
        character.animal,
        character.number_of_legs,
        character.habitat,
        character.animal_name,
        character.favourite_food,
    );
}
