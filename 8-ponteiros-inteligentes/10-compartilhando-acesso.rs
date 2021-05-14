// O RefCell é uma estrutura de dados contâiner comumente mantida
// por ponteiros inteligentes que obtém dados e nos permite emprestar
// referências mutáveis e imutáveis para o que está lá dentro. Ele evita
// o abuso do empréstimo aplicando as regras de segurança de memória do
// Rust em tempo de execução quando você pede emprestado os dados que
// estão dentro:

// Apenas uma referência mutável OU várias referências imutáveis, mas não
// ambas!

// Se você violar essa regra, o RefCell retornará um panic.

use std::cell::RefCell;

struct Pizza {
    fatias: u8
}

impl Pizza {
    fn comer(&mut self) {
        println!("mais saborosa na heap!");
        self.fatias -= 1;
    }
}

fn main() {
    // RefCell valida a segurança da memória em tempo de execução
    // aviso: pizza_cell não é mut
    let pizza_cell = RefCell::new(Pizza{fatias:8});

    {
        // mas podemos emprestar referências mutáveis!
        let mut mut_ref_pizza = pizza_cell.borrow_mut();
        mut_ref_pizza.comer();
        mut_ref_pizza.comer();

        // mut_ref_pizza é descartada no final do escopo
    }

    // agora podemos tomar emprestado imutavelmente
    // uma vez que nossa referência mutável é descartada
    let ref_pizza = pizza_cell.borrow();
    println!("Sobraram {} fatias", ref_pizza.fatias);
}