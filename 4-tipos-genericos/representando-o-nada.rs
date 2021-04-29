// Em outras linguagens, a palavra-chave null é usada para representar a ausência.
// de um valor. Isso cria dificuldades nas linguagens de programação, porque o nosso
// programa possa falhar ao interagir com uma variável/campo.

// O Rust não tem null, mas não ignora a importancia de presentar o nada.
// Considere uma representação ingênua usando uma ferramenta que já conhecemos.

// Esse padrão para fornecer uma representação alternativa ao None
// por um ou vários valores alternados é muito comum em Rust devido a
// falta de um valor null. Tipos genéricos ajudam a resolver esse desafio

enum Item {
    Inventario(String),
    // None representa a ausência de um item
    None,
}

struct Sacola {
    item: Item,
}