// As enumerações permitem criar um novo tipo que pode conter o valor
// de vários elementos etiquetados usando a palavra-chave "enum".

// O "match" ajuda a garantir o tratamento exaustivo de todos os valores possíveis
// de enum, tornando-o uma ferramenta poderosa para garantir um código de qualidade.
#![allow(dead_code)] // Essa linha evita avisos do compilador.

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} é um caranguejo", ferris.name),
        Species::Octopus => println!("{} é um polvo", ferris.name),
        Species::Fish => println!("{} é um peixe", ferris.name),
        Species::Clam => println!("{} é um molusco", ferris.name),
    }
}
