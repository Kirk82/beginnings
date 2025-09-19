use std::collections::HashMap;

use rng::*;

fn main() {
    let rng_generator = Rng::new();

    //picks a random number
    let animal_selector = rng_generator.gen_range(1..4);

    // Assigns the random number to an animal from the Animal Enum
    let choosen_animal = match animal_selector {
        1 => Animal::Cat,
        2 => Animal::Human,
        3 => Animal::Dolphin,
        _ => panic!(),
    };

    //initialises the animal_legs varibale
    let animal_legs;

    //Says how many legs an animal has
    animal_legs = match choosen_animal {
        Animal::Cat => 4,
        Animal::Human => 2,
        Animal::Dolphin => 0,
    };

    //This hashmap assigns different habitats to different animals
    let mut habitat_map = HashMap::new();

    habitat_map.insert(Animal::Cat, vec!["a house", "the wild"]);

    habitat_map.insert(Animal::Human, vec!["a house"]);

    habitat_map.insert(Animal::Dolphin, vec!["the Ocean", "Sea Wrold"]);

    //connects the animal to the chosen animal variable
    let potential_habitat = habitat_map.get(&choosen_animal).unwrap();

    //randomly chooses a habitat
    let habitat = potential_habitat.sample(rng_generator).unwrap();

    //Prints a line in terimal using randomly generated info
    println!(
        "This {:?} has {} legs and lives in {}",
        choosen_animal, animal_legs, habitat
    );

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Animal {
        Cat,
        Human,
        Dolphin,
    }
}
