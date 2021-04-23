// Sentiu falta do operador switch? O Rust tem uma palavra-chave muito
// útil para comparar um valor com todas as condições possíveis e
// executar um bloco de código se a condição for verdadeira. Vejamos
// como isso funciona com números. Falaremos mais a respeito de padrões
// de correspondência com dados mais complexos em capítulos futuros.

// O match é exaustivo, então todos os casos devem ser analisados.

// Correspondência (matchings) combinando com desestruturação é um dos
// padrões mais comuns que você verá no Rust.

fn main() {
    let x = 42;
    match x {
        0 => {
            println!("encontrei o zero");
        }

        // podemos comparar com múltiplos valores.
        1 | 2 => {
            println!("encontrei 1 ou 2!");
        }

        // podemos comparar com iteradores.
        3..=9 => {
            println!("encontrei um número entre 3 e 9, inclusive");
        }

        // podemos vincular a uma variável o número a ser comparado.
        matched_num @ 10..=100 => {
            println!("encontrei o número {} entre 10 e 100!", matched_num);
        }
        // Está é a correspondência padrão se não existir nenhum caso correspondido.
        _ => {
            println!("encontrei outra coisa!");
        }
    }
}