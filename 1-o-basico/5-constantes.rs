// As constantes nos permitem especificar um valor comum que pode ser usado muitas vezes por todo
// nosso código de maneira eficiente. Ao invés de copiar os valores como variáveis onde serão utilizadas
// as constantes substituirão o identificador de texto pelo seu valor onde quer que estejam sendo
// usadas em tempo de compilação.

// Diferenntemente das variáveis, o tipo das constantes devem ser sempre declarados.

// Os nomes das constantes são sempre em SCREAMING_SNAKE_CASE.

const PI: f32 = 3.14159;

fn main() {
    println!("Para fazer uma {}zza, você primeiro deve criar o Universo.", PI);
}