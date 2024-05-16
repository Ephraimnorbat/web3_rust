mod plants;
mod mammals;
mod reptiles;
mod counties;


use plants::{fruits,trees};

use mammals::animals_mammals;
use reptiles::animals_reptiles;
use counties::counties_kenya;

fn main() {
    println!("Hello, world!");
    fruits::print_fruits();
    trees::print_trees();

    animals_mammals();
    animals_reptiles();
    counties_kenya();
}
