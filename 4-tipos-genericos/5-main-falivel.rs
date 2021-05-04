// O main tem a capacidade de retornar um Result!

fn faz_alguma_coisa_que_pode_falhar(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("este é o número correto!"))
    }
}


// O main não retorna um valor, mas pode retornar um erro!
fn main() -> Result<(), String> {
    let result = faz_alguma_coisa_que_pode_falhar(12);
    match result {
        Ok(v) => println!("Encontrei {}", v),
        Err(_e) => {
            // Lida com o erro normalmente
            // retorna um novo erro do main que nos informa
            // o que deu de errado.
            return Err(String::from("Alguma coisa deu errado!"));
        }
    }

    // Observe que usamos um valor unit dentro de um Result OK
    // para representar que tudo está bem.
    Ok(())
}