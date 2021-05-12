// As traits podem ter métodos implementados.

// As funções não têm acesso direto aos campos internos de uma struct,
// mas podem ser úteis para compartilhar comportamentos entre muitos
// implementadores de traits.

struct CriaturaMarinha {
    pub: nome: String,
    barulho: String,
}

impl CriaturaMarinha {
    fn pega_barulho(&self) -> &str {
        &self.barulho
    }
}

trait FazedorBarulho {
    fn faz_barulho(&self);

    fn faz_muito_barulho(&self) {
        self.faz_barulho();
        self.faz_barulho();
        self.faz_barulho();
    }
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
    criatura.faz_muito_barulho();
}