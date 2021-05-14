// O código Rust pode ter uma infinidade de representações dos erros,
// mas a biblioteca padrão tem uma trait universal
// "std::error::Error" para escrever os erros.

// Usando um ponteiro inteligente Box podemos usar o tipo Box<dyn
// std::error::Error> como um tipo comum para retornar os erros,
// porque nos permite propagar um erro na heap e interagir com ele em
// alto nível sem precisar conhecer um tipo específico.

// No início do Tour por Rust, aprendemos que main pode retornar um
// erro. Agora podemos retornar um tipo capaz de descrever quase
// qualqeur tipo de erro que possa ocorrer em nosso programa, desde que
// a estrutura de dados do erro implemente a trait usual Error do Rust.

// fn main() -> Result<(), Box<dyn std::error::Error>>

use core::fmt::Display;
use std::error::Error;

struct Pizza;

#[derive(Debug)]
struct NotFreshError;

impl Display for NotFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Esta pizza não está quente!")
    }
}

impl Error for NotFreshError {}

impl Pizza {
    fn comer(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(NotFreshError))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let heap_pizza = Box::new(Pizza);
    heap_pizza.comer()?;
    Ok(())
}