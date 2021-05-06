// Uma fatia de string é uma referência a uma sequência de bytes na
// memória que deve ser sempre utf-8 válido.

// Uma fatia de string (uma subfatia) de uma fatia str também deve
// ser um utf-8 válido.

// Métodos comuns de &str

//      - len obtém o comprimento da string literal em bytes (não o 
//      número de caracteres).
//      - starts_with / ends_with para testes básicos.
//      - is_empty retorna true se o comprimento for zero.
//      - find retorna um Option<usize> da primeira posição de um 
//      texto

fn main() {
    let a = "oi, rustacean";
    println!("{}", a.len());
    let primeira_palavra = &a[0..2];
    let segunda_palavra = &a[3..7];
    // let meio_caranguejo = &a[3..5]; falha!
    // Rust não aceita fatias de strings inválidos de unicode
    println!("{} {}", primeira_palavra, segunda_palavra);
}