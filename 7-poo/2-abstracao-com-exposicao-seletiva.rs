// O Rust pode esconder o funcionamento interno dos objetos.

// Por padrão, os campos e métodos são acessíveis apenas ao módulo ao
// qual pertencem.

// A palavra-chave `pub` expõe os campos e métodos do struct fora do
// módulo.

struct CriaturaMarinha {
    pub nome: String,
    barulho: String,
}

impl CriaturaMarinha {
    fn pega_barulho() -> &str {
        &self.barulho
    }
}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    println!("{}", criatura.pega_barulho());
}