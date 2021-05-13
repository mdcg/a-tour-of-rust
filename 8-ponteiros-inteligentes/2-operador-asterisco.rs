// O operador * é uma forma explícita de desreferenciamento de uma
// referência.

// let a: i32 = 42;
// let ref_ref_ref_a: &&&i32 = &&&a;
// let ref_a: &i32 = **ref_ref_ref_a;
// let b: i32 = *ref_a;

// Detalhes da memória:
//      - Como i32 é um tipo primitivo que implementa a trait Copy, os
//      bytes da variável a na pilha são copiados para os bytes da
//      variável b

fn main() {
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("{}", b);
}