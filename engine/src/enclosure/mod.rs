use crate::animal;

const ENCLOSURE_CAPACITY: usize = 3;
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

    fn ok_to_add_animal(&self) -> bool {
        let contains_male = self.animal_list.iter().any(|a| a.sex == animal::Sex::Male);
        self.animal_list.len() < ENCLOSURE_CAPACITY && !contains_male
    }

    pub fn add_animal(&mut self, animal: animal::Animal) -> Result<(), String> {
        if self.ok_to_add_animal() {
            self.animal_list.push(animal);
            Ok(())
        } else {
            Err(format!("Enclosure {} is full", self.id))
        }
    }
}
