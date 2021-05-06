// Os especificadores de tempo de vida nos permitem ser explícitos em
// certos cenários que o compilador não pode resolver sozinho,
// distinguindo todas os tempos de vida dos componentes na assinatura
// de uma função.

struct Foo {
    x: i32,
}

// foo_b e o valor de retorno compartilham do mesmo tempo de vida
// foo_a tem um tempo de vida não relacionado.
fn faz_alguma_coisa<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = faz_alguma_coisa(&foo_a, &foo_b);
    // foo_a é descartado aqui porque somente o tempo de vida de
    // foo_b existe além deste ponto
    println!("{}", x);
    // x é descartado aqui
    // foo_b é descartado aqui
}