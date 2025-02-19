// #![warn(missing_docs)]

///! This is the main binary for the pigeonhole game.
///! It will generate a list of animals and a list of enclosures.
///! It will then allow the player to assign animals to enclosures per the rules of the game.
///! This is the main loop of the game.
use engine::enclosure::Enclosure;

fn main() {
    let mut e = engine::generate_initial_animal_list(10);
    let temporary_enclosure: Option<&mut Enclosure> = Some(&mut e);
    let mut el = engine::generate_enclosure_list(10);

    let a = engine::animal::Animal::new(1);
    let target = &mut el[1];
    let _r =
        engine::enclosure::Enclosure::move_animal_into_enclosure(temporary_enclosure, a, target);

    engine::print_enclosure_list(&el);
}
