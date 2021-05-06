// Da mesma forma que as funções. os tipos de dados podem ser
// parametrizados com os especificadores de tempos de vida.

// O Rust valida a estrutura de dados que contém as referências para que
// nunca dure mais do que os proprietários para os quais as suas
// referências apontam.

// Não podemos ter estruturas rodando por aí com referências
// apontando para o nada.

struct Foo<'a> {
    i:&'a i32
}

fn main() {
    let x = 42;
    let foo = Foo {
        i: &x
    };
    println!("{}", foo.i);
}