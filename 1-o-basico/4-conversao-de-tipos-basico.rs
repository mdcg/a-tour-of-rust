// Rust requer que sejamos explícitos quando se trata de tipos numéricos.
// Não podemos usar um u8 quando precisamos de um u32 sem que se produza um erro.

// Por sorte, o Rust faz com que as conversões de tipos numéricos sejam muito simples usando a palavra chave `as`.

fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}