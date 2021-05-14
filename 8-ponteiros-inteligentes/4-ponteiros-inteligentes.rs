// Além da possibilidade de criar referências a dados tipados existentes
// usando o operador &, o Rust nos dá a possibilidade de criar structs
// reference-like chamadas "ponteiros inteligentes".

// Grosso modo, podemos pensar nas referências como um tipo que nos dá
// acesso a outro tipo. Os ponteiros inteligentes são diferentes em seu
// comportamento das referências normais, pois operam com base na
// lógica interna que um programador escreve. (O programador é a parte inteligente)

// Normalmente os ponteiros inteligentes implementam as traits Deref, DerefMut e Drop
// para especificar a lógica do que deve acontecer quando a estrutura é desreferenciada
// com os operadores * e .

use std::ops::Deref;
struct Linguarudo<T> {
    valor: T,
}

impl<T> Deref for Linguarudo<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} foi usado!", std::any::type_name::<T>());
        &self.valor
    }
}

fn main() {
    let foo = Linguarudo {
        valor: "mensagem secreta",
    }

    // O desreferenciamento acontece aqui imediatamente
    // depois do foo é auto-referenciado para a função
    // `len`
    println!("{}", foo.len());
}