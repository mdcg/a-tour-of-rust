// Usando referências vocẽ pode definir o valor do proprietário usando
// o operador *.

// Você também pode obter uma cópia do valor de propriedade usando o
// operador * (Se o valor puder ser copiado. Discutiremos tipos
// copiáveis nos próximos capítulos).

fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // Pega uma cópia do proprietário do valor
    *f = 13; // atribue à referência do proprietário do valor
    println!("{}", bar);
    println!("{}", foo);
}