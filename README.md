# a-tour-of-rust

## Dia 1

### Variáveis

* Rust consegue inferir o tipo da variável em 99% dos casos; 
* Você explicitamente pode especificar o tipo da variável que deseja trabalhar; 
* Rust suporta shadowing; 
* É uma boa prática utilizar o padrão snake_case para especificar nomes de variáveis.

### Modificando valores

* Rust trabalha com dois tipos valores: Mutáveis e Imutáveis; 
* Para valores mutáveis, o compilador permitirá que a variável seja lida e gravada; 
* Para valores imutáives, o compiladores permitirá apenas a leitura da variável; 
* Para marcar uma variável como Mutável, utilize a palavra reservada `mut`.

### Tipos Básicos

* Booleanos: `bool` para representar verdadeiro/falso; 
* Números inteiros sem sinal, para representar números inteiros não negativos: `u8`, `u32`, `u64`, `u128`; 
* Números inteiros com sinal, para reprensetar números inteiros positivos e negativos: `i8`, `i32`, `i64`, `i128`; 
* Ponteiros de números inteiros para representar índices e o comprimento de coleções em memória: `usize`, `isize`; 
* Números de ponto flutuante: `f32`, `f64`; 
* Tuplas para passar sequências de valores fixos para a pilha: `(value, value, ...)`; 
* Matriz: Uma coleção de elementos similares com comprimento fixo conhecidos em tempo de compilação; 
* Slices: Uma coleção de elementos similares com comprimento conhecidos em tempo de execução; 
* String Slice: Texto com comprimento conhecido em tempo de execução.

### Conversão de tipos básicos

* Você deve ser específico ao utilizar valores númericos, pois, por exemplo, você não pode operar um `u8` com um `u32` sem que não aconteça nenhum erro; 
* Você pode declarar variáveis númericas especificando o tipo logo após o número desejado, por exemplo: `10u8`, `10u32`, `10u64`, etc; 
* Rust permite que você converta tipos numéricos utilizando a palavra reservada `as`, por exemplo: 

``` rust
let a = 13u8;
let b = 7u32;
let c = a as u32 + b;
```

## Dia 2

### Constantes

* Constantes são valores que não mudam no código; 
* Para declarar uma constante, utilize a palavra reservada `const`; 
* Diferentemente das variáveis, o tipo das constantes devem sempre ser declarado; 
* Os nomes das constantes são sempre em SCREAMING_SNAKE_CASE.

### Matrizes

* Uma matriz é uma coleção de tamanho fixo onde todos os seus valores possuem o mesmo tipo; 
* Para declarar uma matriz, usamos a notação `[T, N]`, onde T é o tipo dos valores e N o seu comprimento; 
* Os valores dentro de uma matriz podem ser recuperados utilizando `[x]`, onde x é o indice do tipo usize, que sempre começa do zero, do valor que deseja recuperar.

### Funções

* Funções podem receber zero ou N parâmetros; 
* Os valores dos parâmetros devem ser especificados na declaração da função; 
* O valor de retorno também deve ser especificado.

### Retorno de múltiplos valores

* Funções também podem devolver múltiplos valores, retornando sempre uma tupla com os mesmos; 
* elementos da tupla podem ser referenciados pelo seu índice, contudo, ao contrário de matrizes, o acesso aos indices de uma tupla é através do "." (Exemplo: `tup.0`, `tup.1`, ...); 
* Rust também suporta "desestruturação", onde o retorno múltiplo de uma função, pode ser utilizado para atribuir valores a várias variáveis de uma vez só (Exemplo: `let (a, b) = mult_r(123, 654)`).

### Retornando nada

* É possível criar funções que não retornando absolutamente nada; 
* Caso não seja especificado nenhum retorno, A função devolverá uma unit (basicamente uma tupla vazia); 
* Em alguns casos, alguns desenvolvedores podem explicitamente retornar nada, utilizando um `return ()`, mas isso é extremamente incomum.

### Referências interessantes

* [#3 - Basic data types in Rust programming language](https://www.youtube.com/watch?v=n5TRBkbystY)
* [Data Types](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch03-02-data-types.html)
* [Rust Language Cheatsheets - Data Types](https://cheats.rs/#basic-types)

## Dia 3

### if/else if/else

* Condicionais em Rust são similares a qualquer linguagem; 
* Os operadores `==`, `!=`, `<`, `>`, `<=`, `>=`, `!`, `||`,  `&&` são permitidos.

### loop

* É possível criar um loop infinito utilizando o `loop`; 
* Para pará-lo, precisa ter uma instrução `break` dentro dele.

### while

* O `while` permite que você estabeleça facilmente uma condição no loop; 
* O loop será executado até que sua condição verificadora seja `false`.

### for

* Qualquer iterador pode ser utilizado com o `for`; 
* Se você utilizar o operador `..`, o `for` irá percorrer do valor inicial até, mas não incluso, o seu valor final; 
* Se você optar pelo uso do operador `..=`, o `for` irá percorrer do valor inicial até o último, inclusive.

### match

* O `match` é o famoso "switch" de outras linguagens; 
* O `match` é exaustivo, então todos os casos serão analisados; 
* Um padrão muito interessante e comum em estruturas no Rust é utilizar a o `match` justamente com desestruturação; 
* Podemos comparar múltiplos valores em uma mesma instrução utilizando `|`; 
* Podemos comparar intervalos utilizando interadores; 
* Podemos vincular uma variável uma correspondência (Por exemplo: `matched_num @ 10..=100 =>`).

### Referências interessantes

* [https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust](https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust)

-[https://doc.rust-lang.org/std/primitive.slice.html](https://doc.rust-lang.org/std/primitive.slice.html)

* [https://doc.rust-lang.org/book/ch04-03-slices.html](https://doc.rust-lang.org/book/ch04-03-slices.html)
* [https://doc.rust-lang.org/std/vec/struct.Vec.html](https://doc.rust-lang.org/std/vec/struct.Vec.html)

## Dia 4

Dia de prática no Codewars!

## Dia 5

### Retornando valores de um loop

* É possível retornar um valor através do "break" em um loop!
* O Rust possui formas particulares de atribuir a uma variável o valor de retorno em um bloco, mas isso será descrito no próximo capítulo.

### Retornando valores de um bloco de código

* Se a última instrução de um `if`, `match`, função ou bloco de código for uma expressão sem `; `, o Rust vai retorná-la como valor de um bloco; 
* Essa é uma maneira muito legal de trabalhar com uma "lógica concisa" que retorna um valor que pode ser atribuído a uma nova variável; 
* O operador ternário em Rust é feito utilizando essa lógica.

## Dia 6

### Estruturas

* Uma struct nada mais é que uma coleção de dados; 
* Um campo é um valor associado a uma struct, que pode ser tipos primitivos ou até mesmo outras structs; 
* Sua definição é um modelo para o compilador organizar os campos de maneira próximas na memória; 

### Chamando métodos

* Temos duas formas de "chamar métodos". Isso vai depender do tipo de método que estamos trabalhando, que pode ser ou métodos estáticos ou métodos de instância; 
* Métodos estáticos estão associados a um tipo, e são chamados utilizando o operador `::` (Exemplo: `String::from("olá, ferris")` ); 
* Métodos de instância por sua vez estão associados a uma instância de um tipo. Eles são chamados utilizando o operador `.` .

 
 ### Criando dados na memória

* Quando instanciamos uma struct, o compilador cria os campos associados lado-a-lado na memória; 
* Nós instanciamos uma struct informando todos os valores dentro dela (Exemplo: `StructExample {...}` ); 
* Textos entre aspas são somente leitura, por isso, são colocados na região da memória de dados; 
* Uma chamada, por exemplo `String::from("Teste")` cria uma instância que é colocada na pilha lado-a-lado dos campos pertencentes à struct principal; 
* Como um `String::from("Teste")` cria um texto que pode ser alterado, então ele aloca o texto na memória heap, e também, armazena no heap uma referência a esse mesmo local e depois armazena também no `struct String` (Parece um pouco complicado, e é mesmo, haha); 

 
 ### Estuturas tipo Unit

* É possível criar structs que não tenham campos; 
* Não é muito usual, mas geralmente quando são criadas esses tipos de structs, nós chamamos ela de "unit-like", pois são muito parecidas com o `()` que é um unit.

 
 ### Referências

* [https://doc.rust-lang.org/std/vec/struct.Vec.html](https://doc.rust-lang.org/std/vec/struct.Vec.html)
* [https://doc.rust-lang.org/std/primitive.usize.html](https://doc.rust-lang.org/std/primitive.usize.html)

## Dia 7

### Enumerações

* É possível utilizar `enum` em Rust!
* `enum` nos permite criar um tipo com vários elementos etiquetados; 
* Geralmente, `enum`s são muito bons de serem utilizados com o `match`, que ajuda a garantir o tratamento exaustivo de todos os valores possíveis; 
* Esse é um estilo muito bom para criar um código de qualidade.

### Enumerações com dados

* Elementos `enum` podem ter um ou mais tipos de dados; 
* Algo legal a se destacar, é que quando um `enum`, em um `match`, corresponde a um padrão, você pode vincular um nome de variável a cada valor de dados; 
* Um valor de dados em memória terá um tamanho de memória igual ao seu maior elemento; 
* Além dos tipos de dados dos elementos, cada um deles possui um valor numérico que representa a sua "etiqueta"; 
* `enum` em Rust também é conhecido como "tagged-union".

### Referências

* [https://stackoverflow.com/questions/30811107/how-do-i-get-the-first-character-out-of-a-string](https://stackoverflow.com/questions/30811107/how-do-i-get-the-first-character-out-of-a-string)
* [https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust](https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust)
* [https://doc.rust-lang.org/std/primitive.char.html](https://doc.rust-lang.org/std/primitive.char.html)

## Dia 8

### O que são tipos genéricos

* Tipos genéricos nos permitem definir parcialmente um enum ou uma struct deixando que o compilador defina uma versão definitiva em tempo de compilação; 
* Você pode ajudar o compilador a definir o tipo final explicitamente utilizando o operador "turbofish" (Ex: `Sacola::<i32>`); 

### Representando o nada

* Rust não possui valores "nulos", mas não ignora a importância de representar o nada; 
* Para lidar com isso, Rust possui o `None`, que é um tipo "opcional" (na verdade, é um "Option", ler mais [aqui](https://doc.rust-lang.org/std/option/)) para lidar com valores que por ventura podem ser vazios; 

### Referência

* [https://doc.rust-lang.org/std/option/](https://doc.rust-lang.org/std/option/)

## Dia 9

### Option

* Rust possui enumerações genéricas para representarmos valores nulos chamado de `Option`; 
* Existem dois tipos de valores de `Option`: `None` e `Some`; 
* Eles são bastante comuns em códigos Rust e bastante elegantes de serem utilizados em conjunto com o `match`.

### Result

* Muito parecido com o `Option` também temos o enumerador `Result`; 
* `Result` é utilizado mais comumente para representar valores que podem "falhar"; 
* Existem dois valores de `Result`: `Ok` e `Err`; 
* Os tipos `Result` são parametrizados com dois tipos diferentes, um que será utilizado em caso de sucesso e outro em caso de erro (Ex: `enum Result<T, E> {Ok(T), Err(E)}` => `Result<i32, String>`).

## Dia 10

### Main falível

* O main tem a capacidade de retornar em Rust; 
* No caso em específico, podemos retornar um `Result` como `Ok` ou `Err`; 
* Sobre questões de valores, ele não consegue retornar nenhum, geralmente o `Ok` é atribuído como um unit (Usando o `()`) e o `Err` é uma descrição do que aconteceu de errado; 

### Manipulaçao de erros elegantes

* Trabalhar com `Result` no Rust é tão comum que a linguagem possui um operador muito poderoso para trabalhar com ele que é o `?`, por exemplo

``` rust
// Essa declaração
do_somenthing_that_might_fail()?

// é equivalente a essa
match do_somenthing_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
```

### Manipulação de erros (Option e Result) deselegante

* Trabalhar com `Option` e `Result` pode ser um pouca chato, contudo, o Rust tem implementado um método para você obter o valor de `Option` ou `Result` de maneira rápida e "feia" que é o `unwrap`; 
* Se a enumeração for do tipo `None` ou `Err`, teremos um `panic`.

``` rust
// Essa declaração
my_option.unwrap()

// é equivalente a essa
match my_option {
    Some(v) => v,
    None => panic!("Alguma mensagem de erro gerada pelo Rust!"),
}
```

### Matrizes

* Um dos tipos genéricos mais úteis é o `Vec`; 
* Ele é utilizado para representar uma matriz, que por sua vez, é uma lista de tamanho variável; 
* Podemos criar `Vec` de forma explicita (Usando turbofish ou declaração por inferência) ou utilizarmos a macro `vec!`; 
* Podemos iterar nos dados do `Vec` utilizando um `for`, chamando o método `iter`; 
* Internamente, o `Vec` é uma struct, mas ele contém uma referência na heap onde estão seus itens; 
* Inicialmente, um `Vec` começa com uma capacidade padrão. Se essa capacidade aumentar, ele realoca os elementos na heap para ter uma capacidade maior.

### Referências

- [https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1](https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1)
- [https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse)
- [https://doc.rust-lang.org/book/ch08-01-vectors.html](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [https://doc.rust-lang.org/std/primitive.char.html](https://doc.rust-lang.org/std/primitive.char.html)
- [https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)