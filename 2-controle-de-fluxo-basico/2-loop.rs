// Precisando de um loop infinito?

// O break nos permite sair de um loop quando precisar.

// O loop tem um segredo do qual falaremos em breve.

fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
}