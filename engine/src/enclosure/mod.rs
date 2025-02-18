use crate::animal;

const ENCLOSURE_CAPACITY: usize = 3;
#[derive(Debug, Clone)]
pub struct Enclosure {
    pub id: u32,
    pub animal_list: Vec<animal::Animal>,
}

impl Enclosure {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            animal_list: Vec::with_capacity(ENCLOSURE_CAPACITY),
        }
    }

    fn animal_is_in_list(
        animal_to_be_added: &animal::Animal,
        list_to_check: &Vec<animal::Animal>,
    ) -> bool {
        for a in list_to_check.iter() {
            if a.id == animal_to_be_added.id {
                return true;
            }
        }
        false
    }

    fn ok_to_add_animal(&self, animal_to_add: &animal::Animal) -> bool {
        self.animal_list.len() < ENCLOSURE_CAPACITY
            && !(Enclosure::animal_is_in_list(animal_to_add, &self.animal_list))
    }

    pub fn add_animal(&mut self, animal: animal::Animal) -> Result<(), String> {
        if self.ok_to_add_animal(&animal) {
            self.animal_list.push(animal);
            Ok(())
        } else {
            Err(format!("Enclosure {} is full", self.id))
        }
    }
}
