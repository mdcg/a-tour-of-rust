// Os ponteiros inteligentes podem parecer limitados, mas eles podem
// fazer algumas combinações muito poderosas.

// Rc<Vec<>Foo> - Permite a clonagem de vários ponteiros inteligentes
// que podem pegar emprestado o mesmo vetor de estruturas de dados imutáveis
// na heap.

// Rc<RefCell<Foo>> - Permite múltiplos ponteiros inteligentes a
// capacidade de emprestar mutável/imutavelmente a mesma estrutura
// Foo.

// Arc<Mutex<Foo>> - Permite que vários ponteiros inteligentes
// bloqueiem empréstimos mutáveis/imutáveis temporários
// exclusivamente por thread de CPU.

// Detalhes da memória:

// - Você notará um assunto em comum em muitas dessas
// combinações. O uso de um tipo de dado imutável (possivelmente
// pertencente a vários ponteiros inteligentes) para modificar dados
// internos. Isso é conhecido como o padrão "mutabilidade interior"
// no Rust. É um padrão que nos permite dobrar as regras de uso de
// memória em tempo de execução com o mesmo nível de segurança
// das verificações de tempo de compilação do Rust.

use std::cell::RefCell;
use std::rc::Rc;

struct Pizza {
    fatias: u8,
}

impl Pizza {
    fn comer_fatia(&mut self, nome: &str) {
        println!("{} pegou uma fatia!", nome);
        self.fatias -= 1;
    }
}

struct CriaturaMarinha {
    nome: String,
    pizza: Rc<RefCell<Pizza>>,
}

impl CriaturaMarinha {
    fn comer(&self) {
        // usa um ponteiro inteligente da pizza para um empréstimo mutável
        let mut p = self.pizza.borrow_mut();
        // dá uma mordida!
        p.comer_fatia(&self.nome);
    }
}

fn main() {
    let pizza = Rc::new(RefCell::new(Pizza { fatias: 8}));
    // ferris e sarah recebem clones do ponteiro inteligente da pizza
    let ferris = CriaturaMarinha {
        nome: String::from("ferris"),
        pizza: pizza.clone(),
    };
    let sarah = CriaturaMarinha {
        nome: String::from("sarah"),
        pizza: pizza.clone(),
    };
    ferris.comer();
    sarah.comer();

    let p = pizza.borrow();
    println!("sobraram {} fatias", p.fatias);
}