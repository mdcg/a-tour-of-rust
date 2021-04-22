// Se num tipo for especificado para o retorno de uma função, ele devolve uma tupla vazia,
// também conhecida como "unit".

// Uma tupla vazia é representada por ().

// O uso de () é incomum, mas é necessário saber o seu significado uma vez que aparecerá de vez
// em quando.

fn faz_nada() -> () {
    return ();
}

// O tipo de retorno é implicitamente ()
fn faz_nada2() {
    // esta função irá devolver () se não for especificado
    // para retornar
}

fn main() {
    let a = faz_nada();
    let b = faz_nada2();

    // Exibindo uma string de depuração para a e b
    println!("O valor de a: {:?}", a);
    println!("O valor de b: {:?}", b);
}