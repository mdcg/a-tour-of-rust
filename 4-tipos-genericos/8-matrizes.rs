// Alguns dos tipos genéricos mais úteis são os tipos de coleção. Uma
// matriz é uma lista de itens de tamanho variável representada pela
// estrutura "Vec"

// A macro "vec!" nos permite criar facilmente uma matriz ao invés de
// construir uma manualmente.

// Vec possui o método iter() o qual cria um iterador a partir de 
// uma matriz, permitindo-nos facilmente usar uma matriz em um loop
// for.

// Detalhes de memória:

// - Vec é um struct, mais internamente contém uma referência a
// uma lista fixa de seus itens na heap.
// - Uma matriz começa com uma capacidade padrão. Quando são 
// adicionados mais itens do que a capacidade inicial, ele realoca
// seus dados na heap para ter uma nova lista fixa com capacidade
// maior

fn main() {
    // Podemos ser explícitos com o tipo
    let mut i32_vec = Vec::<i32>::new(); // Turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    
    // Rust também consegue determinar o tipo automaticamente
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // Olha essa macro!
    let string_vec = vec![String::from("olá"), String::from("mundo")];
    for word in string_vec.iter() {
        println!("{}", word);
    }
}