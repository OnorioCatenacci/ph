use crate::animal;

const ENCLOSURE_CAPACITY: usize = 3;
const TEMPORARY_ENCLOSURE_ID: u32 = 0;

#[derive(Debug)]
///! Enclosure is a struct that represents an enclosure in the zoo.  Each enclosure is given a unique id for ease of reference.
pub struct Enclosure {
    /// The id of the enclosure.  This is a unique identifier for each enclosure.  Id 0 is reserved for the temporary enclosure.
    pub id: u32,
    /// The animals in the enclosure.  This is a vector of animals that are in the enclosure.  The vector is limited to a maximum of 3 animals.
    pub animals_in_enclosure: Vec<animal::Animal>,
}

impl Enclosure {
    /// Generate a new "enclosure" with the specified id.
    pub fn new(id: u32) -> Enclosure {
        if id == TEMPORARY_ENCLOSURE_ID {
            Enclosure {
                id,
                animals_in_enclosure: Vec::<animal::Animal>::new(),
            }
        } else {
            Enclosure {
                id,
                animals_in_enclosure: Vec::<animal::Animal>::with_capacity(ENCLOSURE_CAPACITY),
            }
        }
    }
}

enum EnclosureError {
    /// The animal is not in the enclosure
    NotInEnclosure,
    /// Cannot test for animal in enclosure because the enclosure is empty
    EnclosureIsEmpty,
    /// The animal was found in the enclosure
    AnimalFoundInEnclosure,
}

fn is_animal_in_enclosure(
    animal_to_check: &animal::Animal,
    enclosure_to_be_checked: &Enclosure,
) -> EnclosureError {
    if enclosure_to_be_checked.animals_in_enclosure.is_empty() {
        return EnclosureError::EnclosureIsEmpty;
    } else {
        for a in enclosure_to_be_checked.animals_in_enclosure.iter() {
            if a.id == animal_to_check.id {
                return EnclosureError::AnimalFoundInEnclosure;
            }
        }
        return EnclosureError::NotInEnclosure;
    }
}

fn remove_animal_from_enclosure(enclosure: &mut Enclosure, animal_to_remove: animal::Animal) -> () {
    if enclosure.animals_in_enclosure.is_empty() {
        return;
    } else {
        let mut index_to_remove = 0;
        for (i, a) in enclosure.animals_in_enclosure.iter().enumerate() {
            if a.id == animal_to_remove.id {
                index_to_remove = i;
                break;
            }
        }
        enclosure.animals_in_enclosure.remove(index_to_remove);
    }
}

fn ok_to_add_animal(animal_to_add: &animal::Animal, enclosure_to_be_checked: &Enclosure) -> bool {
    //Temporary enclosure can hold any number of animals
    if enclosure_to_be_checked.id == TEMPORARY_ENCLOSURE_ID {
        return true;
    } else {
        let animal_in_enclosure =
            match is_animal_in_enclosure(&animal_to_add, &enclosure_to_be_checked) {
                EnclosureError::AnimalFoundInEnclosure => true,
                EnclosureError::EnclosureIsEmpty => false,
                EnclosureError::NotInEnclosure => false,
            };
        enclosure_to_be_checked.animals_in_enclosure.len() < ENCLOSURE_CAPACITY
            && !animal_in_enclosure
    }
}

///! move_animal_into_enclosure is a function that moves an animal from one enclosure to another.
///! It will return an error if the animal is already in the target enclosure or if the target enclosure is full.
pub fn move_animal_into_enclosure(
    source: Option<&mut Enclosure>,
    animal_to_move_into_enclosure: animal::Animal,
    target: &mut Enclosure,
) -> Result<(), String> {
    if ok_to_add_animal(&animal_to_move_into_enclosure, &target) {
        match source {
            Some(s) => remove_animal_from_enclosure(s, animal_to_move_into_enclosure),
            None => (),
        }
        target
            .animals_in_enclosure
            .push(animal_to_move_into_enclosure);
        return Ok(());
    } else {
        return Err(format!(
            "Animal {} is already in the target list",
            animal_to_move_into_enclosure.id
        ));
    }
}
