// Funções podem devolver múltiplos valores retornando uma tupla de valores.

// Os elementos da tupla podem ser referenciados pelo seu índice.

// O Rust suporta vários modos de desestruturação que veremos em diferentes formas,
// permitindo extrair subconjuntos de dados de maneira prática.

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // Retorna uma tupla de valores
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // Desestrutura a tupla para duas variáveis
    let (a, b) = swap(result.0, result.1);
}