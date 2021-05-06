// A macro format! nos permite criar uma string definindo uma string
// parametrizada com espaços reservados para onde e como os valores
// devem ser colocados (por exemplo, {}).

// O format! usa as mesmas strings parametrizadas que o println!.

// Os recursos desta função têm um escopo muito maior do que é abordado aqui.
// Mais informações em https://doc.rust-lang.org/std/fmt/

fn main() {
    let a = 42;
    let f = format!("o segredo da vida: {}", a);
    println!("{}", f);
}