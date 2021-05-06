// Uma String é uma estrutura que contém uma sequência de bytes
// utf-8 na memória heap.

// Como sua memória está na pilha, ela pode ser estendida, modificada
// etc. de tal maneira que os literais de string não podem.

// Métodos comuns:

//      - push_str para adicionar mais bytes utf-8 ao final de uma string
//      - replace para substituir sequências de bytes utf-8 por outras.
//      - to_lowercase / to_uppercase para alterações de maiúscula e minúsculas.
//      - trim para cortar espaços.

// Quando uma String é descartada, sua memória heap também é descartada.

// O tipo String possui um operador + que estende a string com um &str e retorna
// a si mesmo, mas não pode ser tão ergonômico quanto você espera.

fn main() {
    let mut helloworld = String::from("Olá");
    helloworld.push_str(" mundo");
    helloworld = helloworld + "!";
    println!("{}", helloworld);
}