use std::collections::HashMap;

use rng::*;

fn main() {
    let rng_generator = Rng::new();

    let animal_selector = rng_generator.gen_range(1..4);

    let choosen_animal = match animal_selector {
        1 => Animal::Cat,
        2 => Animal::Human,
        3 => Animal::Dolphin,
        _ => panic!(),
    };

    let animal_legs;

    animal_legs = match choosen_animal {
        Animal::Cat => 4,
        Animal::Human => 2,
        Animal::Dolphin => 0,
    };

    let mut habitat_map = HashMap::new();

    habitat_map.insert(Animal::Cat, vec!["a house", "the wild"]);

    habitat_map.insert(Animal::Human, vec!["a house"]);

    habitat_map.insert(Animal::Dolphin, vec!["the Ocean", "Sea Wrold"]);

    let potential_habitat = habitat_map.get(&choosen_animal).unwrap();

    let habitat = potential_habitat.sample(rng_generator).unwrap();

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
