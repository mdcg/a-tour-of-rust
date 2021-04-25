// loop podem ser interrompidos para retornar um valor.

fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "encontrei o 13";
        }
    };
    println!("do loop: {}", v);
}