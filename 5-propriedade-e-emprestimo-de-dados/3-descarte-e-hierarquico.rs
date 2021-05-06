// Quando uma estrutura é descartada, a estrutura em si é descartada
// primeiro, depois os filhos são descartados individualmente e assim
// por diante

// Detalhes da Memória:

//      - Ao liberar automaticamente a memória, o Rust ajuda a garantir
//      que haja menos vazamento de memória;
//      - Os recursos de memória só podem ser descartados uma vez.

struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo é descartado primeiro e só então foo.bar é descartado.
}