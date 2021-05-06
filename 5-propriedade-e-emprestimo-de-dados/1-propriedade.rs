// Instanciar um tipo e vinculá-lo a um nome de variável cria um
// recurso de memória que o compilador Rust validará por toda sua
// vida útil. A variável vinculada é chamada de proprietária do recurso.

struct Foo {
    x: i32,
}


fn main() {
    // Instanciamos structs e vinculamos a variáveis
    // para criar recursos de memória.
    let foo = Foo { x: 42 };
    // foo é o proprietário
}