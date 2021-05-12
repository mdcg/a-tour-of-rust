// Os métodos são executados de duas maneiras:

//      - static dispatch - Quando o tipo da instância é conhecido, temos
//      conhecimento direto de qual função chamar.
//      - dynamic dispatch -  Quando o tipo da instância não é conhecido
//      precisamos descobrir uma maneira de chamar a função correta.

// Os tipos de trait `&dyn MinhaTrait` nos permite a habilidade de trabalhar
// com instâncias de objetos indiretamente usando dispatch dinâmico.

// Quando o dispatch dinâmico é usado, o Rust irá encorajar você a pôr o
// dyn antes do seu tipo da trait, assim os outros ficarão cientes.

// Detalhes de memória:

//      - O dispatch dinâmico é um pouco mais lento por causa da procura
//      que o ponteiro realiza para localizar a verdadeira chamada da 
//      função.

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

fn static_faz_barulho(criatura: &CriaturaMarinha) {
    // sabemos o tipo real
    criatura.faz_barulho();
}

fn dynamic_faz_barulho(fazedor_barulho: &dyn FazedorBarulho) {
    // não sabemos o tipo real
    fazedor_barulho.faz_barulho();
}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    static_faz_barulho(&criatura);
    dynamic_faz_barulho(&criatura);
}