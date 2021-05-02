// Rust possui uma enumeração genérica chamada Result que nos
// permite retornar um valor que tem a possibilidade de falhar.

// Esta é a maneira idiomática pela qual a linguagem faz a manipulação
// de erros

// enum Result<T, E> {
//     Ok(T),
//     Err(T),
// }

// Observe que nossos tipos genéricos possuem vários tipos
// parametrizados separados por vírgula.

// Essa enumeração é tão comum que instâncias delas podem ser
// criadas em qualquer lugar usando a palavra-chave Ok e Err.

fn faz_alguma_coisa_falhar(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("este não é um número correto"))
    }
}

fn main() {
    let result = faz_alguma_coisa_falhar(12);
    // o match nos permite desconstruir o Result elegantemente 
    // e garante que lidemos com todos os casos!
    match result {
        Ok(v) => println!("Encontrei {}", v),
        Err(e) => println!("Erro {}", e),
    }
}