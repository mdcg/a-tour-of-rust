// O operador . é usado para acessar campos e métodos de uma
// referência. Ele funciona de uma maneira mais sutil.

// let f = Foo { value: 42 };
// let ref_ref_ref_f = &&&f;
// println!("{}", ref_ref_ref_f.value);

// Porque não precisamos adicionar *** antes de
// ref_ref_ref_f? Isso ocorre porque o operador . desreferencia
// automaticamente uma sequência de referências. Essa última linha é
// transformada na seguinte pelo compilador automaticamente.

// println!("{}", (***ref_ref_ref_f).value);

struct Foo {
    value: i32
}

fn main() {
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);
}