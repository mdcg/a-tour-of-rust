// As referências nos permitem emprestar o acesso a um recurso com o operador &.

// As referências também são descartadas do mesmo jeito que outros recursos.

struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f é descartado aqui
    // foo é descartado aqui
}