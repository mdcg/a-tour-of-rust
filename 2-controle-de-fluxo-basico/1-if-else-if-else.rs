// A ramificação de código em Rusut não otem nenhuma novidade.

// Não há parêntesis nas condições! Realmente precisamos deles? Assim a nossa lógica
// fica mais simples e limpa.

// Todos os nossos operadores relacionais e lógicos habituais contiuam funcionando
// ==, !=. <, >, <= ,>=, !, ||, &&

fn main() {
    let x = 42;
    if x < 42 {
        println!("Menor que 42");
    } else if x == 42 {
        println!("Igual a 42");
    } else {
        println!("Maior que 42");
    }
}