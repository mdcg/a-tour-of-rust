// Mesmo que o Rust nem sempre nos mostre no código, o compilador
// conhece o tempo de vida de cada variável e tentará validar uma
// referência que nunca deve existir por mais tempo que seu
// proprietário.

// As funções podem ser explícitas parametrizando a assinatura da
// função com símbolos que ajudam a identificar quais parâmetros e
// valores de retorno compartilham o mesmo tempo de vida.

// Os especificadores de tempo de vida sempre começam com um '
// (por exemplo 'a, 'b, 'c)

struct Foo {
    x: i32,
}

// O parâmetro foo e valor de retorno compartilham o mesmo tempo de vida.
fn faz_alguma_coisa<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo =  Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x é descartada aqui permitindo-nos criar uma referência não-mutável
    let y = faz_alguma_coisa(&foo);
    println!("{}", y);
    // y é descartada aqui
    // foo é descartada aqui
}