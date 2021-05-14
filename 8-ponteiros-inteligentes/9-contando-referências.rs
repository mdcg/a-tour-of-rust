// o Rc é um ponteiro inteligente que move os dados da pilha para a
// heap. Ele nos permite clonar outros ponteiros inteligentes Rc que
// têm a capacidade de imutavelmente tomar emprestado os dados que foram
// colocados na heap.

// Somente quando o último ponteiro inteligente é descartado que os
// dados na heap são desalocados.

use std::rc::Rc;

struct Pizza;

impl Pizza {
    fn comer(&self) {
        println!("mais saborosa na heap!")
    }
}

fn main() {
    let heap_pizza = Rc::new(Pizza);
    let heap_pizza2 = heap_pizza.clone();
    let heap_pizza3 = heap_pizza2.clone();

    heap_pizza3.comer();
    heap_pizza2.comer();
    heap_pizza.comer();

    // toda a contagem das referências dos
    // ponteiros inteligentes são descartados
    // agora que os dados de Pizza na heap são
    // finalmente desalocados.
}