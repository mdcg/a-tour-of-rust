// O Mutex é uma estrutura de dados container comumente mantida
// por ponteiros inteligentes que recebe os dados e nos permite
// emprestar referências mutáveis e imutáveis aos dados que estão
// dentro. Isso evita o abuso do empréstimo fazendo com que o sitema
// operacional restrinja o acesso aos dados a apenas uma thread original
// seja concluída com seu empréstimo bloqueado.

// O multithreading está além do escopo do Tour por Rust, mas Mutex é
// uma parte fundamental da orquestração de várias threads de CPU
// acessando os mesmos dados.

// Há um ponteiro inteligente especial Arc que é identico ao Rc,
// exceto pelo uso de incrementes thread-safe de contagens de
// referência. Frequentemente é usado para ter muitas referências ao
// mesmo Mutex.

use std::sync::Mutex;

struct Pizza;

impl Pizza {
    fn comer(&self) {
        println!("Só eu como a pizza agora!");
    }
}

fn main() {
    let mutex_pizza = Mutex::new(Pizza);
    // vamos pegar emprestada uma referência imutável bloqueada de Pizza
    // temos que desembrulhar o resultado de um bloqueio porque pode falhar
    let ref_pizza = mutex_pizza.lock().unwrap();
    ref_pizza.comer();
    // a referência bloqueada é descartada aqui
    // e o valor protegido da mutex pode ser usado por outra pessoa.
}