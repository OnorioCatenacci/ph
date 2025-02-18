#![warn(missing_docs)]

/// This is the main binary for the pigeonhole game.
/// It will generate a list of animals and a list of enclosures.
/// It will then allow the player to assign animals to enclosures per the rules of the game.
/// This is the main loop of the game.
fn main() {
    let al = engine::generate_initial_animal_list(10);
    let mut el = engine::generate_enclosure_list(10);

    if let Some(a) = al.iter().nth(1) {
        for e in el.iter_mut() {
            let _r = e.add_animal(*a);
        }
    }

    //    engine::print_animal_list(&al);
    engine::print_enclosure_list(&el);
}
