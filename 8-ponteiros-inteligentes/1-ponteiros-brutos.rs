// As referências podem ser convertidas em um tipo mais primitivo
// chamado de "ponteiro bruto". Muito parecido com um número, ele pode
// ser copiado e movido com poucas restrições. O Rust não garante a
// validade da localização da memória para o qual aponta.

// Há dois tipos de indicadores brutos:
//      - *const T - Um ponteiro bruto para dados do tipo T que nunca
//      deve mudar.
//      - *mut T - Um ponteiro bruto para dados do tipo T que podem
//      mudar.

// Os ponteiros brutos podem ser convertidos de e para números (por
// exemplo, usize).

// Os ponteiros brutos podem acessar dados com código inseguro
// (falaremos a respeito mais tarde).

// Detalhes da memória:
//      - Uma referência em Rust é muito semelhante a um ponteiro em C
//      em termos de uso, mas com muito mais restrições de tempo de
//      compilação sobre como pode ser armazenado e movido para outras
//      funções
//      - Um ponteiro bruto em Rust é semelhante a um ponteiro em C,
//      pois representa um número que pode ser copiado, passado e até
//      transformado em tipos numéricos onde pode ser modificado
//      como um número para fazer cálculos de ponteiros.

fn main() {
    let a = 42;
    let localizacao_de_memoria = &a as *const i32 as usize;
    println!("O dado está aqui {}", localizacao_de_memoria)
}