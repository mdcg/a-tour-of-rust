// Box é uma estrutura de dados que nos permite mover nossos dados
// da stack para a heap.

// Box é uma estrutura conhecida como ponteiro inteligente que
// contém o ponteiro dos nossos dados na heap.

// Como o Box é uma struct com tamanho conhecido (porque apenas
// contém um ponteiro), é frequentemente usada como uma maneira de
// armazenar uma referência a algo em uma struct que deve saber o
// tamanho de seus campos.

// O Box é tão comum que pode ser usado de qualquer lugar:

// Box::new(Foo { ... })

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

struct Oceano {
    animais: Vec<Box<dyn FazedorBarulho>>,
}

fn main() {
    let ferris = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    let sarah = CriaturaMarinha {
        nome: String::from("Sarah"),
        barulho: String::from("zum"),
    };
    let oceano = Oceano {
        animais: vec![Box::new(ferris), Box::new(sarah)],
    }
    for a in oceano.animais.iter() {
        a.faz_barulho();
    }
}