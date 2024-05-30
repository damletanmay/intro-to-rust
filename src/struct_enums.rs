use std::vec;

pub fn main() {
    explaination();
}

fn leave_space() {
    println!("");
    println!("");
    println!("");
}

fn explaination() {
    // enums
    #[derive(Debug)]
    enum Creatures {
        Dog { name: String },
        Parrot { name: String },
        Crow { name: String },
        Wolf { name: String },
        Cat,
    }

    // implementation block for creatures
    impl Creatures {
        // similar to a constructor
        fn new(animal_type: String, animal_name: String) -> Option<Creatures> {
            if animal_type.to_lowercase() == "dog".to_string() {
                Some(Creatures::Dog { name: animal_name })
            } else if animal_type.to_lowercase() == "parrot".to_string() {
                Some(Creatures::Parrot { name: animal_name })
            } else if animal_type.to_lowercase() == "crow".to_string() {
                Some(Creatures::Crow { name: animal_name })
            } else if animal_type.to_lowercase() == "wolf".to_string() {
                Some(Creatures::Wolf { name: animal_name })
            } else if animal_type.to_lowercase() == "cat".to_string() {
                Some(Creatures::Cat)
            } else {
                None
            }
        }

        // a summon creature method which will call the creature
        fn summon_creature(creature: &Self) {
            match creature {
                Self::Dog { name } => {
                    println!("{} says Woof! Woof!", name);
                }
                Self::Parrot { name } => {
                    println!("{} says Hey I'm A Parrot!", name);
                }
                Self::Crow { name } => {
                    println!("Crow {} sees all that you do", name)
                }
                Self::Cat => {
                    println!("Your Cat *Ignores*");
                }
                _ => {
                    println!("You Don't have any pets :(");
                }
            }
        }
    }

    let my_pet: Option<Creatures> = Creatures::new("Dog".to_string(), "Chester".to_string());

    match my_pet {
        Some(pet) => {
            println!("My pet:{:?}", pet);
            Creatures::summon_creature(&pet);
        }
        None => println!("Problem in creating a pet"),
    }

    leave_space();

    // shadowing to make a cat
    let my_pet = Creatures::new("Parrot".to_string(), "Robert".to_string());

    Creatures::summon_creature(&my_pet.unwrap());

    leave_space();

    // struct
    #[derive(Debug)]
    struct Gods {
        name: String,
        all_creatures: Option<Vec<Creatures>>,
        weapons: Vec<String>,
    }

    impl Gods {
        fn new(all_creatures: Vec<Creatures>, name: String, weapons: Vec<String>) -> Option<Gods> {
            if all_creatures.len() > 0 || weapons.len() > 0 {
                Some(Gods {
                    all_creatures: Some(all_creatures),
                    name,
                    weapons,
                })
            } else {
                None
            }
        }
    }

    let odin_pet_1: Option<Creatures> = Creatures::new("crow".to_string(), "Huginn".to_string());
    let odin_pet_2 = Creatures::new("crow".to_string(), "Muninn".to_string());
    let mut odin_pets: Vec<Creatures> = vec![];

    match odin_pet_1 {
        Some(pet_1) => odin_pets.push(pet_1),
        None => {}
    }

    match odin_pet_2 {
        Some(pet_2) => odin_pets.push(pet_2),
        None => {}
    }

    let odin_weapons = vec!["Gungnir".to_string(), "Draupnir".to_string()];

    let odin = Gods::new(odin_pets, "Odin".to_string(), odin_weapons);

    println!("Odin: {:?}\n", odin);

    match &odin {
        Some(odin) => match &odin.all_creatures {
            Some(all_creatures) => {
                for creature in all_creatures.iter() {
                    Creatures::summon_creature(creature);
                }
            }
            None => {}
        },
        None => {}
    }

    leave_space();

    let thor_weapons = vec!["Mjolnir".to_string()];
    let thor = Gods::new(vec![], "Thor".to_string(), thor_weapons);

    // Creatures::summon_creature(&thor.unwrap().all_creatures.unwrap()[0]); panics as no creatures are there

    println!("{:?}", thor);

    leave_space();

    // tuple struct
    #[derive(Debug)]
    struct Giants(String);

    let loki = Giants("Loki".to_string());
    let angrboða = Giants("Angrboða".to_string());
    let jörmungandr = Giants("Jörmungandr".to_string());
    let fenrir = Giants("Fenrir".to_string());
    let hel = Giants("Hel".to_string());

    println!("The God of Mischeif :{:?}", loki);
    println!("The Mistress of Loki:{:?}", angrboða);
    println!("The Midgard Serpent:{}", jörmungandr.0);
    println!("The Keeper of Hellheim:{}", hel.0);
    println!("The Wolf at the Gates of Hell:{}", fenrir.0);

    // leave_space();

    // TODO: Make a God called kratos using new method and he should have
    // 2 wolfs called Speki and svenna,
    // 2 weapons - Leviathan Axe & Blades of Chaos
    // print kratos

    // leave_space();

    // Make a Dwarf Tuple Struct
    // and make 5 dwarfs brok, sindri, durlin, ivalid, fafnir, andvari
    // print all dwarfs
}
