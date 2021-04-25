// if, match, funções e blocos de código têm maneiras únicas de
// retornar valores em Rust.

// Se a última instrução de um if, match, função ou bloco de código for uma
// expressão sem ";", o Rust vai retorná-la como um valor do bloco.

// Esta é uma ótima maneira de criar uma lógica concisa que retorna um
// valor que pode ser atribuída a uma nova variável.

// Note isso também permite que o operador if funcione como uma expressão
// ternária concisa.
fn example() -> i32 {
    let x = 42;
    // expressão ternária em Rust
    let v = if x < 42 { -1 } else { 1 };
    println!("do if: {}", v);

    let food = "hambúrguer";
    let result = match food {
        "cachorro-quente" => "é cachorro-quente",
        // repare que as chaves são opcionais quando o retorno é só uma expressão
        _ => "não é cachorro-quente",
    };
    println!("identificado o lanche: {}", result);

    let v = {
        // Este bloco de código nos permite obter um resultado sem poluir o escopo
        // da função
        let a = 1;
        let b = 2;
        a + b
    };
    println!("do bloco: {}", v);

    // Assim retornamos um valor em Rust no final de uma função.
    v + 4
}

fn main() {
    println!("da função: {}", example());
}