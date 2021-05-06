// Muitos tipos podem ser convertidos em um string usando to_string.

// A função genérica parse pode ser usada para converter strings ou 
// literais de strings em um valor digitado. Esta função retorna um
// Result porque pode falhar.

fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}