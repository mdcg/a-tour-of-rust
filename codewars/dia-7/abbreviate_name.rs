// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust

// Minha solução
fn first_letter(name: &str) -> String {
    return name.chars().nth(0).unwrap().to_uppercase().to_string();
}

fn abbrev_name(name: &str) -> String {
    let full_name: Vec<&str> = name.split(" ").collect();
    format!("{}.{}", first_letter(full_name[0]), first_letter(full_name[1]))
}
// Melhor solução
fn abbrev_name(name: &str) -> String {
    name.split(' ')
    .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
    .collect::<Vec<_>>()
    .join(".")
}

// Eu Gostei bastante dessa
