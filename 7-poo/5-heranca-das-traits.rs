// As traits podem herdar métodos de outras traits.

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

trait LoudFazedorBarulho: FazedorBarulho {
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

impl LoudFazedorBarulho for CriaturaMarinha {}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    criatura.faz_muito_barulho();
}