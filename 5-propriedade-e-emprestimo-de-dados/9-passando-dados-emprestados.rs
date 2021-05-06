// As regras do Rust para referências podem ser melhor resumidas
// assim:

//      - O Rust permite que haja apenas uma referência mutável ou
//      múltiplas referências não-mutáveis, mas não ambas.
//      - Uma referência nunca deve viver mais do que o seu proprietário.

// Isso não costuma ser um problema ao passar referências a funções.

// Detalhes da memória:

//      - A primeira regra das referências previnem os data race.
//      O que é um data race? Basicamente isso acontece quando,
//      na leitura de dados, temos a possibilidade de conflito
//      com a existência de outras escritas de dados ao mesmo tempo.
//      Isso acontece frequentemente na programação multithread.
//      - A segunda regra de referências evita o uso indevido de
//      referências que se referem a dados inexistentes (chamados
//      ponteiros pendentes em C).

struct Foo {
    x: i32,
}

fn faz_alguma_coisa(f: &mut Foo) {
    f.x += 1;
    // a referência mutável é descartada aqui
}

fn main() {
    let mut foo = Foo { x: 42 };
    faz_alguma_coisa(&mut foo);
    // porque todas as referências mutáveis são descartadas dentro
    // da função faz_alguma_coisa. Então podemos criar outra referência.
    faz_alguma_coisa(&mut foo);
    // foo é descartado aqui
}