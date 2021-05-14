// Considere alguns ponteiros inteligentes que já vimos como Vec<T>
// e String.

// O Vec<T> é um ponteiro inteligente que simplesmente possui uma
// região da memória em bytes. O compilador Rust não tem ideia do que
// existe nesses bytes. O ponteiro inteligente interpreta o que significa
// pegar os itens da região da memória que ele gerencia, mantém o
// controle de onde os bytes das estruturas de dados começam e terminam
// e, finalmente, desreferencia um ponteiro bruto para uma estrutura de dados
// com um referência ergonômica, limpa e fácil para nós usarmos (por exemplo, my_vec[3]).

// Da mesma forma, String mantém o controle de uma região de memória em bytes,
// restringe programaticamente o conteúdo escrito nele para ser sempre utf-8 válido,
// e ajuda a desreferenciar essa região de memória em um tipo &str.

// Ambas as estruturas de dados usam desreferenciamento inseguro de ponteiros
// brutos para fazer seu trabalho.

// Detalhes da memória:
//      - O Rust tem um equivalente ao malloc do C usando alloc e
//      Layout para obter suas próprias regiões de memória para gerenciar.

use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pizza {
    receita_secreta: usize,
}

impl Pizza {
    fn new() -> Self {
        // Vamos pedir por 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // alocar e salvar a localização da memória como um número
            let ptr = alloc(layout) as *mut u8;
            // use a aritmética de ponteiros e
            // escreva alguns valores u8 na memória
            prt.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pizza { receita_secreta: ptr as usize }
        }
    }
}

impl Deref for Pizza {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interprete o ponteiro receita_secreta como um ponteiro bruto f32
        let ponteiro = self.receita_secreta as *const f32;
        // desreferencie em um valor de retorno &f32
        unsafe { &*ponteiro }
    }
}

fn main() {
    let p = Pizza::new();
    // "Faça uma pizza" desreferenciando o nosso ponteiro inteligente
    // da struct Pizza
    println!("{:?}", *p);
}