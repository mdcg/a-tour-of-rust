// O Result é tão comum que o Rust tem o poderoso operador `?` para
// trabalhar com ele. Estas duas declarações são equivalentes:

//      do_somenthing_that_might_fail()?

//      match do_somenthing_that_might_fail() {
//          Ok(v) => v,
//          Err(e) => return Err(e),
//      }

fn faz_alguma_coisa_que_pode_falhar(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("este não é o número correto"))
    }
}

fn main() -> Result<(), String> {
    // Olha quanto código salvamos!
    let v = faz_alguma_coisa_que_pode_falhar(42)?;
    println!("encontrei {}", v);
    Ok(())
}