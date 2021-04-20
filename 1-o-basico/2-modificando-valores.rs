// Em Rust é importante levar em consideração quais valores serão alterados.
// Os valores se dividem em dois tipos:

// - Mutáveis: O compilador permitirá que a variável seja lida e gravada;
// - Imutáveis: O compilador permitirá apenas a leitura da variável.

// Valores mutáveis são marcados com a palavra-chave `mut`.

fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}