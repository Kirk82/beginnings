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

    println!("A {:?} has {} legs", choosen_animal, animal_legs);

    #[derive(Debug)]
    enum Animal {
        Cat,
        Human,
        Dolphin,
    }
}
