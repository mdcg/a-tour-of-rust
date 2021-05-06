// Referências podem ser usadas até mesmo em pedaços de referências

struct Foo {
    x: i32,
}

fn faz_alguma_coisa(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x é descartado aqui permitindo-nos criar uma referência não mutável
    let y = faz_alguma_coisa(&foo);
    println!("{}", y);
    // y é descartado aqui
    // foo é descartado aqui
}