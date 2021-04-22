// Uma função pode receber zero ou mais parâmetros.

// Neste exemplo, add recebe dois parâmetros do tipo i32 (número inteiro de 32 bits).

// Os nomes das funções são sempre em snake_case.

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("{}", add(42, 13));
}