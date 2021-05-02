// Rust possui uma enumeração genérica embutida chamada
// Option que nos permite representar valores nulos sem precisar 
// usar o null.

// enum Option<T> {
//     None,
//     Some(T),
// }

// Essa enumeração é tão comum que suas instâncias podem ser
// criadas em qualquer lugar com a palavra-chave Some ou None.

// Um tipo struct parcialmente definido
struct Sacola<T> {
    item: Option<T>,
}

fn main() {
    // Nota: Uma sacola para i32, contendo nada! Precisamos especificar
    // o tipo porque o Rust não saberá qual o tipo que sacola é.
    let i32_sacola = Sacola::<i32> { item: None };

    if i32_sacola.item.is_none() {
        println!("Não tem nada na sacola");
    } else {
        println!("Tem alguma coisa na sacola!");
    }

    let i32_sacola = Sacola::<i32> { item: Some(42) };
    if i32_sacola.item.is_some() {
        println!("Tem alguma coisa na sacola!");
    } else {
        println!("Não tem nada na sacola");
    }

    match i32_sacola.item {
        Some(v) => println!("encontrei {} na sacola!", v),
        None => println!("não encontrei nada!"),
    }
}