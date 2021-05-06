// Também podemos emprestar um acesso mutável a um recurso
// usando o operador &mut.

// O proprietário de um recurso não pode ser movido ou modificado
// durante um empréstimo mutável.

// Detalhes de memória:

// - Rust evita ter duas maneiras de alterar um valor que tenha um
// dono, pois introduz a possibilidade de um data race.

struct Foo {
    x: i32,
}

fn faz_alguma_coisa(f: Foo) {
    println!("{}", f.x);
    // f é descartado aqui
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // FAILURE: faz_alguma_coisa_(foo) irá falhar porque
    // foo não pode ser movido furante um empréstimo mutável
    // FAILURE: foo.x = 13; irá falhar porque
    // foo não é modificável durante um empréstimo mutável
    f.x = 13;
    // f é descartado por não ser mais usado depois deste ponto

    println!("{}", foo.x);

    // Isto funciona agora porque todas as referências mutáveis
    // foram descartadas.
    foo.x = 7;

    // move a propriedade de foo para uma função
    faz_alguma_coisa(foo);
}