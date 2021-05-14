// Os ponteiros inteligentes tem a usar código inseguro com bastante
// frequência. Como mencionado anteriormente, eles são ferramentas
// comuns para interagir com níveis mais baixos de memória no Rust.

// O que é um código inseguro? O código inseguro se comporta
// exatamente como Rust normal, com exceção de algumas habilidades
// sobre as quais o compilador Rust é incapaz de oferecer garantias.

// Uma habilidade primária do código inseguro é o "desreferenciamento
// de um ponteiro bruto". Isso significa pegar um ponteiro bruto de uma
// posição na memória, declarar "há uma estrutura de dados aqui!" e
// transformá-lo em uma representação de dados que você pode usar
// (ou seja, *const u8 em u8).

// O Rust não tem como rastrear o significado de cada byte que é gravado
// na memória. E porque o Rust não pode dar garantias sobre o que
// existe em um número arbitrário usado como um ponteiro bruto que
// ele põe o desreferenciamento em um bloco unsafe {}.

// Ponteiros inteligentes desreferenciam ponteiros brutos
// extensivamente, mas são bem competentes no que fazem.

fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];
    // Isto é um ponteiro bruto. Obter o endereço de memória
    // de algo como número é totalmente seguro
    let ponteiro_a = &a as *const u8 as usize;
    println!("Localização dos dados na memória: {}", ponteiro_a);
    // Transformar nosso número em um ponteiro bruto
    // para f32 também é seguro.
    let ponteiro_b = ponteiro_a as *const f32;
    let b = unsafe {
        // Isso não é seguro porque estamos dizendo ao compilador
        // para assumir que o nosso ponteiro é um f32 válido e que
        // desreferencie seu valor para a variável b. O Rust
        // não tem como verificar se essa suposição é verdadeira.
        *ponteiro
    };
    println!("Eu juro que isso é uma pi.zza! {}", b);
}