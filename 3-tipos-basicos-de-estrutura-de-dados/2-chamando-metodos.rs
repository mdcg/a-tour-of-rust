// Diferentemente das funções, os métodos são uma função associada a
// um tipo de dados específico.

// Métodos estáticos: Os métodos que pertencem a um tipo determinado são chamados
// usando o operador ::.

// Métodos de instância: Os métodos que pertencem a um instância de um tipo são
// chamados usando o operador ..

fn main() {
    // Usando um método estático para criar uma instância de String.
    let s = String::from("Hello world!");
    // Usando um método de instância.
    println!("{} is {} characters long.", s, s.len());
}