// O Rust suporta o polimorfismo com traits. As traits nos permitem
// associar um conjunto de métodos a um tipo de struct.

// Primeiro definimos as assinaturas dos métodos de uma trait
// internamente:

// trait MinhaTrait {
//     fn foo(&self);
//     ...
// }

// Quando uma estrutura implementa uma trait, ela estabelece um
// contrato que nos permite interagir indiretamente com a struct
// por meio da trait (por exemplo, &dyn Minha trait) sem precisar
// conhecer o tipo real.

// Os métodos da trait implementados na struct são definidos dentro de 
// um bloco de implementação:

// impl MinhaTrait for MinhaStruct {
//     fn foo(&self) {
//         ...
//     }
//     ...
// }

struct CriaturaMarinha {
    pub nome: String,
    barulho: String,
}

impl CriaturaMarinha {
    fn pega_barulho(&self) -> &str {
        &self.barulho
    }
}

trait FazedorBarulho {
    fn faz_barulho(&self);
}

impl FazedorBarulho for CriaturaMarinha {
    fn faz_barulho(&self) {
        println!("{}", &self.pega_barulho());
    }
}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    println!("{}", criatura.pega_barulho());
}