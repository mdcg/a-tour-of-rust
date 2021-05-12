// O Rust suporta o conceito de um objeto que é uma estrutura associada
// a algumas funções (também conhecidas como métodos).

// O primeiro parâmetro de qualquer método deve ser uma referência à
// instância associada à chamada de método  (por exemplo,
// `instanceOfObj.foo()`). O Rust usa:

// - &self - referência imutável da instância;
// - &mut self - referência mutável da instância.

// Os métodos são definidos dentro de um bloco de implementação com
// a palavra-chave impl.

// impl MinhaStruct {
//     ...
//     fn foo(&self) {
//         ...
//     }
// }

struct CriaturaMarinha {
    barulho: String,
}

impl CriaturaMarinha {
    fn paga_barulho(&self) -> &str {
        &self.barulho
    }
}

fn main() {
    let criatura = CriaturaMarinha {
        barulho: String::from("glub"),
    };

    println!("{}", criatura.paga_barulho());
}