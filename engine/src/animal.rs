use rand::Rng;

///! Sex is an enumeration of the possible sexes of an animal[^note].
///! [^note] I realize that there are intersex animals but this is a simplifiction for purposes of this game.
///! For sake of this game there are MALE and FEMALE animals only.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Sex {
    /// A male animal
    Male,
    /// A female animal
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

///! Animal is a struct that represents an animal in the zoo.  Each animal is given a unique id for ease of reference.
#[derive(Debug, Copy, Clone)]
pub struct Animal {
    /// The id of the animal.  This is a unique identifier for each animal.  There is no special id reserved for any particular animal.
    pub id: u32,
    /// The sex of the animal in question.
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

    #[cfg(test)]
    /// Generate a new "animal" with a specific sex.
    /// This is useful for testing and should only be used for tests.
    pub fn new_specific_sex(id: u32, sex: Sex) -> Self {
        Self { id, sex }
    }
}
