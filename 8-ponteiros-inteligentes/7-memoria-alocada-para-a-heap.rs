// O Box é um ponteiro inteligente que nos permite mover dados da
// pilha para a heap.

// O desreferenciamento nos permite usar os dados alocados na heap
// ergonomicamente como se fossem do tipo original.
struct Pizza;

impl Pizza {
    fn comer(&self) {
        println!("mais sabora na heap!")
    }
}

fn main() {
    let heap_pizza = Box::new(Pizza);
    heap_pizza.comer();
}