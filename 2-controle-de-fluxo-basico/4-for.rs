// O loop for do Rust é uma grande melhoria, uma  vez que ele itera sobre valores
// de qualquer expressão que se avalie como um iterador. O que é um iterador? Um
// iterador é um objeto ao qual você pode perguntar "qual o seu próximo item?"
// até que não haja mais itens.

// Iremos explorar mais isso em um capítulo futuro, mas por enquanto basta
// saber que em Rust é fácil criar iteradores que geram uma sequência de números
// inteiros.

// O operador .. cria um iterador que gera uma sequência a partir de um número inicial até,
// mas não incluso, um número final.

// O operador ..= cria um iterador que gera uma sequência a partir de um número
// inicial até um número final, inclusive.

fn main() {
    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }
}