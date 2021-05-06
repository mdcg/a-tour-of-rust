// As strings no Rust são multilinha por padrão.
// Use um \ no final da linha se você não quiser uma quebra de linha.

fn main() {
    let haiku: &'static str = "
            Ah, o passado.
    O tempo onde se acumularam
            os dias lentos.
        - Busson";
    println!("{}", haiku);

    println!("Olá \
    mundo"); // perceba que o espaço antes de "mundo" é ignorado.
}