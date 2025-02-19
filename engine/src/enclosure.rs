use crate::animal;

const ENCLOSURE_CAPACITY: usize = 3;
const TEMPORARY_ENCLOSURE_ID: u32 = 0;

#[derive(Debug)]
pub struct Enclosure {
    pub id: u32,
    pub animals_in_enclosure: Vec<animal::Animal>,
}

impl Enclosure {
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

    fn is_animal_in_enclosure(
        animal_to_check: &animal::Animal,
        enclosure_to_be_checked: &Enclosure,
    ) -> bool {
        if enclosure_to_be_checked.animals_in_enclosure.is_empty() {
            return false;
        } else {
            for a in enclosure_to_be_checked.animals_in_enclosure.iter() {
                if a.id == animal_to_check.id {
                    return true;
                }
            }
            false
        }
    }

    fn remove_animal_from_enclosure(
        enclosure: &mut Enclosure,
        animal_to_remove: animal::Animal,
    ) -> () {
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

    fn ok_to_add_animal(&self, animal_to_add: animal::Animal) -> bool {
        //Temporary enclosure can hold any number of animals
        if self.id == TEMPORARY_ENCLOSURE_ID {
            return true;
        } else {
            self.animals_in_enclosure.len() < ENCLOSURE_CAPACITY
                && !(Enclosure::is_animal_in_enclosure(&animal_to_add, self))
        }
    }

    pub fn move_animal_into_enclosure(
        source: Option<&mut Enclosure>,
        animal_to_move_into_enclosure: animal::Animal,
        target: &mut Enclosure,
    ) -> Result<(), String> {
        if Enclosure::is_animal_in_enclosure(&animal_to_move_into_enclosure, &target) {
            return Err(format!(
                "Animal {} is already in the target list",
                animal_to_move_into_enclosure.id
            ));
        } else {
            if Enclosure::ok_to_add_animal(&target, animal_to_move_into_enclosure) {
                match source {
                    Some(s) => {
                        Enclosure::remove_animal_from_enclosure(s, animal_to_move_into_enclosure)
                    }
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
    }
}
