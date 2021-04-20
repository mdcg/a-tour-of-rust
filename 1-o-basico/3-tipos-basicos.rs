// Rust tem uma variedade de tipos que lhe serão familiares:
// - Booleanos: `bool` para representar verdadeiro/falso;
// - Números inteiros sem sinal, para representar números inteiros não negativos: `u8`, `u32`, `u64`, `u128`;
// - Números inteiros com sinal, para reprensetar números inteiros positivos e negativos: `i8`, `i32`, `i64`, `i128`;
// - Ponteiros de números inteiros para representar índices e o comprimento de coleções em memória: `usize`, `isize`;
// - Números de ponto flutuante: `f32`, `f64`;
// - Tuplas para passar sequências de valores fixos para a pilha: `(value, value, ...)`;
// - Matriz: Uma coleção de elementos similares com comprimento fixo conhecidos em tempo de compilação;
// - Slices: Uma coleção de elementos similares com comprimento conhecidos em tempo de execução;
// - String Slice: Texto com comprimento conhecido em tempo de execução.

// Textos podem ser mais complexos do que você está acostumado com outras linguagens. Uma vez que o Rust
// é uma linguagem de programação de sistemas, ele cuida do gerenciamento de memória de uma maneira que pode
// você pode não estar familiarizado. Entraremos em detalhes mais adiante.

// Tipos numéricos podem ser especificados explicitamente adicionando o tipo ao final do número (por exemplo: `13u32`, `2u8`).

fn main() {
    let x = 12; // Infere o tipo i32
    let a = 12u8;
    let b = 4.3; // Infere o tipo f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "Olá, mundo!";
    println!("{} {} {} {} {} {} {} {}", x, a, b, c, bv, t.0, t.1, sentence)
}