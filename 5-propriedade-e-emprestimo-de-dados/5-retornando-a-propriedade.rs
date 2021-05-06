// A propriedade também pode ser retornada de uma função.
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // a propriedade é retornada
}

fn main() {
    let foo = do_something();
    // foo se torna o proprietário
    // foo é descartado por causa do fim do escopo da função
}