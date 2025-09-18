use rng::*;

fn main() {
    let rng_generator = Rng::new();

    let first_value = rng_generator.gen_range(0..2);
    let second_value = rng_generator.gen_range(0..2);
    let total = first_value * second_value;

    let total_in_words = match total {
        0 => TotalInWords::EatPie,
        1 => TotalInWords::Mmmm,
        2 => TotalInWords::ILikeDonuts,
        3 => TotalInWords::Help,
        4 => TotalInWords::Me,
        _ => panic!(),
    };

    #[derive(Debug)]
    enum TotalInWords {
        EatPie,
        Mmmm,
        ILikeDonuts,
        Help,
        Me,
    }

    println!("value {:?} = {}", total_in_words, total);

    let animal_selector = rng_generator.gen_range(1..2);

    let choosen_animal = match animal_selector {
        1 => Animal::Cat,
        2 => Animal::Human,
        _ => panic!(),
    };

    let animal_leg_needed: Animal;

    let animal_legs = match animal_leg_needed {
        Animal::Cat => 4,
        Animal::Human => 2,
        _ => panic!(),
    };

    println!("{:?} has legs", choosen_animal);

    #[derive(Debug)]
    enum Animal {
        Cat,
        Human,
    }
}
