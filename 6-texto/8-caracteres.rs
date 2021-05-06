// Com tanta dificuldade para trabalhar com Unicode, o Rust oferece
// uma maneira de recuperar uma sequência de bytes utf-8 como um
// vetor de caracteres do tipo char.

// Um char sempre tem 4 bytes de comprimento (permitindo uma
// pesquisa eficiente de caracteres individuais).

fn main() {
    // guarde os caracteres em vetor de char
    let chars = "oi, rustacean".chars().collect::<Vec<char>>();
    println!("{}", chars.len());
    // visto que chars tem 4 bytes, podemos converter um char em um u32
    println!("{}", chars[3] as u32);
}