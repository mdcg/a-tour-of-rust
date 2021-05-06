// Asd strings são sempre Unicode.

// O tipo de literais String é &'static str:

//      - & significa que está referindo a uma posição na memória e a
//      falta de um &mut significa que o compilador não permitirá
//      modificações.

//      - 'static significa que os dados da string estarão disponíveis
//      até o final da execução do programa (isto é, nunca será
//      descartada).

//      - str significa que aponta para uma sequência de bytes que
//      serão sempre uft-8 válidos.

// Detalhes da memória:

//      - O compilador Rust provavelmente colocará sua string no
//      segmento de dados da memória do seu programa.

fn main() {
    let a: &'static str = "olá, 🦀";
    println!("{} {}", a, a.len());
}