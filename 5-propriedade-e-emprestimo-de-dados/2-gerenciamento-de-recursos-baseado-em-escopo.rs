// O Rust usa o final do escopo como o lugar para desconstruir e
// desalocar um recurso. O termo para esta desconstrução e desalocação
// é chamado descarte (ou drop).

// Detalhes da memória:
//      - O Rust não tem garbage collection
//      - Também conhecido por Aquisição de Recursos e Inicialização
//      (Sigla RAII, em inglês)

struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);
    // foo_a é descartado aqui porque não é mais
    // usado depois deste lugar

    println!("{}", foo_b.x);
    // foo_b é descartado aqui porque é o fim
    // do escopo da função.
}