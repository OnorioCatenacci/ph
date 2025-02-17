use rand::Rng;

/// Sex is an enumeration of the possible sexes of an animal.
/// I realize that there are intersex animals but this is a simplifiction for purposes of this game.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Sex {
    Male,
    Female,
}

fn random_sex() -> Sex {
    let rng = rand::rng().random_range(0..2);
    if rng == 0 {
        Sex::Male
    } else {
        Sex::Female
    }
}

/// Animal is a struct that represents an animal in the zoo.  Each animal is given a unique id for ease of reference.
#[derive(Debug, Copy, Clone)]
pub struct Animal {
    pub id: u32,
    pub sex: Sex,
}

impl Animal {
    /// Generate a new "animal" with a random sex assigned to it.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            sex: random_sex(),
        }
    }

    /// Generate a new "animal" with a specific sex.
    /// This is useful for testing and should only be used for tests.
    pub fn new_specific_sex(id: u32, sex: Sex) -> Self {
        Self { id, sex }
    }
}
