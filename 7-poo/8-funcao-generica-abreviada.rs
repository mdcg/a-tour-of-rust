// O Rust possui uma abreviação para expressar genéricos restritos por
// uma trait:

// fn minha_funcao(foo: impl Foo) {
//     ...
// }

// Isso é equivalente a:

// fn minha_funcao<T>(foo: T)
// where
//     T: Foo
// {
//     ...
// }

struct CriaturaMarinha {
    pub nome: String,
    barulho: String,
}

impl CriaturaMarinha {
    pub fn pega_barulho(&self) -> &str {
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

fn generic_faz_barulho(criatura: &impl FazedorBarulho) {
    // conhecemos o tipo real em tempo de compilação
    criatura.faz_barulho();
}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    generic_faz_barulho(&criatura);
}