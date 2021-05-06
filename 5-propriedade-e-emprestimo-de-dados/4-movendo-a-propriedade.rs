// Quando um proprietário é passado como argumento para uma função,
// a propriedade é movida para o parâmetro da função.

// Após um movimento, a variável na função original não pode ser usada.

// Detalhes da memória:

// - Durante uma movimentação, a memória da pilha do proprietário
// do valor é copiada para a memória da pilha dos parâmetros da
// função chamada

struct Foo {
    x: i32,
}

fn faz_alguma_coisa(f: Foo) {
    println!("{}", f.x);
    // f é descartado aqui
}

fn main() {
    let foo = Foo { x: 42 };
    // foo é movido para faz_alguma_coisa
    faz_alguma_coisa(foo);
    // foo não pode mais ser usado
}