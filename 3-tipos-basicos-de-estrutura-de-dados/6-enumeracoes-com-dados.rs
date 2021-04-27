// Os elementos enum também podem ter um ou mais tipos de dados
// permitindo que eles se comportem como o union da linguagem C.

// Quando um enum corresponde ao padrão usando match, você pode 
// vincular um nome de variável para cada valor de dados.

// Detalhes de memória do enum:

//      - Um valor de dados enum terá um tamanho de memória igual ao
// seu maior elemento. Isso permite que todos os valores possíveis
// caibam no mesmo espaço de memória.
//      - Além dos tipos de dados do elemento (se houver), cada elemento
// também possui um valor numérico que representa qual etiqueta
// (tag) ele é.

// Outros detalhes:

//      - O enum do Rust também é conhecido como tagged-union
//      - A combinação de tipos para criar um noovo tipo é a o que nos
// referimos quando dizemos que Rust tem tipos algébricos.

#![allow(dead_code)] // esta linha evita avisos do compilador

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // os dados de SeaCreature estão na pilha
    let ferris = SeaCreature {
        // A struct da String também estão na pilha,
        // mas mantém uma referência dos dados na heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "grandes",
                        Size::Small => "pequenas",
                    };
                    println!("Ferris é um caranguejo com {} garras {}", num_claws, size_description)
                },
                _ => println!("Ferris é um caranguejo com outro tipo de arma")
            }
        },
        _ => println!("Ferris é outro tipo de animal"),
    }
}