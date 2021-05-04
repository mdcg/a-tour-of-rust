// Trabalhar com Option / Result pode ser entediante quando você
// está apenas tentando escrever um código rápido. Tanto Option
// quanto Result têm uma função chamada "unwrap" que pode ser
// útil para obter um valor de maneira rápida e feia. unwrap irá:

//      1 - Obter o valor de Option / Result
//      2 - Se a enumeração for do tipo None / Err, panic!

// Esses dois trechos de código são equivalentes:

//      my_option.unwrap()

//      match my_option {
//          Some(v) => v,
//          None => panic!("Alguma mensagem de erro gerada pelo Rust!"),
//      }

//      my_result.unwrap()

//      match my_result {
//          Ok(v) => v,
//          Err(e) => panic!("alguma mensagem de erro gerada pelo Rust!"),
//      }

// Se um bom rustacean e uso o match apropriadamente quando puder

fn faz_alguma_coisa_que_pode_falhar(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from(este não é o número correto!))
    }
}

fn main() -> Result((), String) {
    // conciso, mas pretencioso e falha rápido
    let v = faz_alguma_coisa_que_pode_falhar(42).unwrap();
    println!("encontrei {}", v);

    let v = faz_alguma_coisa_que_pode_falhar(1).unwrap();
    println!("encontrei {}", v);

    Ok(())
}