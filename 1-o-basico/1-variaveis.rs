// Variáveis são declaradas usando a palavra-chave let.

// Quando atribuimos um valor, o Rust poderá inferir o tipo da sua variável em 99% dos casos.

// Se não for possível, você poderá especificar o tipo ao declarar a variável.

// Veja que podemos atribuir valores às variáveis várias vezes. Isto é chamado de "shadowing"
// e podemos mudar seu tipo para referências futuras a esse nome.

// Os nomes das variáveis são sempre em `snake_case`.

fn main() {
    // Rust infere o tipo de x
    let x = 13;
    println!("{}", x);

    // Rust também pode ser explícito com o tipo
    let x: f64 = 3.14159;
    println!("{}", x);

    // Rust também pode declarar primeiro e inicializar depois, mas é incomum
    let x;
    x = 0;
    println!("{}", x);
}