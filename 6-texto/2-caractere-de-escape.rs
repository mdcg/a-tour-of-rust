// É um desafio representar visualmente certos caracteres, portanto
// os códigos de escape nos permitem colocar um símbolo em seu lugar.

// O Rust suporta códigos de escape comuns de linguagens baseadas em C:

//      - \n - newline
//      - \r - carriage return
//      - \tab - tab
//      - \\ - backslash
//      - \0 - null
//      - \' - single-quote

fn main() {
    let a: &'static str = "Ferris diz: \t\"olá!\"";
    println!("{}", a);
}