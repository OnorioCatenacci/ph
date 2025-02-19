#![warn(missing_docs)]

//! Game engine for a zoo simulation
//!
//! This is a simple game engine for a zoo simulation. It allows you to create animals and enclosures, and assign animals to enclosures based on certain rules.
//! We've abstracted the animals and the enclosures into their own modules.
//! The animals are created with a random sex, and the enclosures have a maximum capacity of 3 animals.

use enclosure::Enclosure;

pub mod animal;
pub mod enclosure;

/// This function generates an initial list of animals to assign to enclosures.
/// This will generate a list of animals with the specified number of animals and return it.
/// The sexes of the animals are generated at random.
///
/// Example
/// ```rust
/// let temporary_enclosure = engine::generate_initial_animal_list(10);
/// assert_eq!(temporary_enclosure.animals_in_enclosure.len(), 10);
/// ```
pub fn generate_initial_animal_list(how_many: u32) -> Enclosure {
    // We want an immutable vector of animals
    let animal_list: Vec<animal::Animal> = {
        let mut t_animals: Vec<animal::Animal> = Vec::new();
        for i in 0..how_many {
            let t_animal = animal::Animal::new(i);
            t_animals.push(t_animal);
        }
        t_animals
    };
    let mut temporary_enclosure = Enclosure::new(0);
    for a in animal_list {
        let _r = Enclosure::move_animal_into_enclosure(None, a, &mut temporary_enclosure);
    }
    temporary_enclosure
}

/// This function will take a list of animals and print the list.
/// This will print the list of animals to the console.
///
/// Example
/// ```rust
/// let temporary_enclosure = engine::generate_initial_animal_list(10);
/// engine::print_animal_list(&temporary_enclosure.animals_in_enclosure);
/// ```
pub fn print_animal_list(animal_list: &Vec<animal::Animal>) {
    for a in animal_list {
        println!("Animal {:?}", a);
    }
}

/// This function will take a list of enclosures and print the list.
/// This will print the list of enclosures to the console.
/// For each enclosure it will print a list of animals in the enclosure.
pub fn print_enclosure_list(enclosure_list: &Vec<enclosure::Enclosure>) {
    for e in enclosure_list {
        println!("Enclosure {:?}", e);
        let al = &e.animals_in_enclosure;
        println!("Animal {:?}", al);
    }
}

/// This function generates a list of containing a specified number of enclosures.
///
/// Example
/// ```rust
/// use engine::generate_enclosure_list;
/// let enclosure_list = generate_enclosure_list(10);
/// assert_eq!(enclosure_list.len(), 9); // We subtract 1 because we have a temporary enclosure that is not in the list
/// ```
pub fn generate_enclosure_list(how_many: u32) -> Vec<enclosure::Enclosure> {
    let enclosure_list: Vec<enclosure::Enclosure> = {
        let mut t_enclosures: Vec<enclosure::Enclosure> = Vec::new();
        for i in 1..how_many {
            t_enclosures.push(enclosure::Enclosure::new(i));
        }
        t_enclosures
    };
    enclosure_list
}

#[cfg(test)]
mod engine_tests {
    use super::*;

    const ANIMAL_LIST_SIZE: u32 = 10;
    const ENCLOSURE_LIST_SIZE: u32 = ANIMAL_LIST_SIZE;

    #[test]
    fn length_of_animal_list_is_correct() {
        let enclosure = generate_initial_animal_list(ANIMAL_LIST_SIZE);
        assert_eq!(
            enclosure.animals_in_enclosure.len(),
            ANIMAL_LIST_SIZE as usize
        );
    }

    #[test]
    fn animal_list_is_not_empty() {
        let enclosure = generate_initial_animal_list(ANIMAL_LIST_SIZE);
        assert!(!enclosure.animals_in_enclosure.is_empty());
    }

    #[test]
    fn animal_list_is_empty() {
        let enclosure = generate_initial_animal_list(0);
        assert!(enclosure.animals_in_enclosure.is_empty());
    }

    #[test]
    fn animal_ids_are_indexed_correctly() {
        let enclosure = generate_initial_animal_list(ANIMAL_LIST_SIZE);
        assert!(
            enclosure.animals_in_enclosure[0].id == 0 && enclosure.animals_in_enclosure[9].id == 9
        )
    }

    #[test]
    fn all_animals_must_be_male_or_female() {
        let enclosure = generate_initial_animal_list(1000);
        let mut result = true;
        for a in enclosure.animals_in_enclosure.iter() {
            result = result & (a.sex == animal::Sex::Male || a.sex == animal::Sex::Female);
        }
        assert!(result);
    }

    #[test]
    fn length_of_enclosure_list_is_correct() {
        let el = generate_enclosure_list(ENCLOSURE_LIST_SIZE);
        // We subtract 1 because we have a temporary enclosure that is not in the list
        assert_eq!(el.len(), (ENCLOSURE_LIST_SIZE - 1) as usize);
    }

    #[test]
    fn enclosure_list_is_not_empty() {
        let el = generate_enclosure_list(ENCLOSURE_LIST_SIZE);
        assert!(!el.is_empty());
    }

    #[test]
    fn enclosure_list_is_empty() {
        let el = generate_enclosure_list(0);
        assert!(el.is_empty());
    }

    #[test]
    fn cannot_assign_more_than_three_animals_to_an_enclosure() {
        let mut enclosure = enclosure::Enclosure::new(1);
        let source_enclosure = generate_initial_animal_list(4);
        for a in source_enclosure.animals_in_enclosure.iter() {
            let result = Enclosure::move_animal_into_enclosure(None, *a, &mut enclosure);
            if result.is_err() {
                assert_eq!(
                    result.unwrap_err(),
                    format!("Animal {} is already in the target list", a.id)
                );
            }
        }
    }

    #[test]
    fn can_assign_three_females_to_an_enclosure() {
        let mut enclosure = enclosure::Enclosure::new(1);
        let a1 = animal::Animal::new_specific_sex(1, animal::Sex::Female);
        let a2 = animal::Animal::new_specific_sex(2, animal::Sex::Female);
        let a3 = animal::Animal::new_specific_sex(3, animal::Sex::Female);
        _ = Enclosure::move_animal_into_enclosure(None, a1, &mut enclosure);
        _ = Enclosure::move_animal_into_enclosure(None, a2, &mut enclosure);
        let result = Enclosure::move_animal_into_enclosure(None, a3, &mut enclosure);
        assert!(result.is_ok());
    }

    #[test]
    fn we_should_be_able_to_put_each_animal_into_a_separate_enclosure() {
        let source_enclosure = generate_initial_animal_list(ENCLOSURE_LIST_SIZE);
        let mut el = generate_enclosure_list(ENCLOSURE_LIST_SIZE);
        for (a, e) in source_enclosure
            .animals_in_enclosure
            .into_iter()
            .zip(el.iter_mut())
        {
            let result = Enclosure::move_animal_into_enclosure(None, a, e);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn show_me_the_animal_list() {
        let enclosure = generate_initial_animal_list(ANIMAL_LIST_SIZE);
        print_animal_list(&enclosure.animals_in_enclosure);
    }
}
