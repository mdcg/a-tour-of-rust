// Quando nós instanciamos uma struct no nosso código o programa
// cria os campos associados lado-a-lado na memória.

// Nós instanciamos uma estrutura especificando todos os valores dos
// campos dentro de StructName {...}

// Os campos são acessados usando o operador de ponto `.`.

// Detalhes da memória do nosso exemplo:

// - O texto dentro das aspas é somente leitura (por exemplo, "Ferris"),
// portanto é colocado na região da memória de dados.

// - A chamada da função String::from cria uma struct String que é colocada
// lado-a-lado com os campos de SeaCreature na pilha. Uma String representa
// um texto que pode ser alterado e faz assim:

//      1 - Criando memória no heap para o texto onde ele pode ser modificado
//      2 - Armazenando uma referência a esse local de memória no heap e armazenando-o
//      no struct String (mais a respeito em lições futuras).

// - Finalmente, nossos dois amigos Ferris e Sarah têm estruturas de dados que sempre terão
// locais fixos em nosso programa, portanto, eles são colocados na pilha.
struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    // Os dados de SeaCreature estão na pilha
    let ferris = SeaCreature {
        animal_type: String::from("caranguejo"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("garra"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("polvo"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("nenhum"),
    };

    println!("{} é um {}. Eles têm {} braços, {} patas, e uma arma de {}", ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);
    println!("{} é um {}. Eles têm {} braços e {} patas. Eles não tem armas...", sarah.name, sarah.animal_type, sarah.arms, sarah.legs);
}