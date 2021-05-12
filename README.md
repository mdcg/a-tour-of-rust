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

* [https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1](https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1)
* [https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse)
* [https://doc.rust-lang.org/book/ch08-01-vectors.html](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* [https://doc.rust-lang.org/std/primitive.char.html](https://doc.rust-lang.org/std/primitive.char.html)
* [https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

## Dia 11

### Propriedade

* Quando instanciamos um tipo e o vinculamos a um nome de variável, criamos um recurso de memória que o compilador Rust validará por toda vida útil; 
* A variável a qual vinculamos o tipo é chamada de "proprietária".

### Gerenciamento de Recursos Baseados em Escopo

* Rust não possui Garbage Collection; 
* Basicamente, o Rust usa o fim do escopo como o lugar para desconstruir e desalocar um recurso; 
* Essa ação é chamada de "drop".

### O Descarte é hierárquico

* Quando criamos uma struct, no momento que ela é descartada, a struct pai é dropada primeiro, depois seus filhos são dropados individualmente, e assim por diante; 

### Movendo a Propriedade

* Quando um proprietário é passado como argumento para uma função, a propriedade é movida para o parâmetro da função; 
* Após esse movimento, a variável na função original não pode mais ser usada.

### Retornando a Propriedade

* A propriedade também pode ser retornada de uma função.

### Emprestando Propriedades com Referências

* As referências nos permitem emprestar o acesso a um recurso com o operador `&`; 

### Emprestando Propriedades Mutáveis usando Referências

* Também podemos emprestar acesso mutável a um recurso usando o operador `&mut`; 
* O proprietário de um recurso não pode ser movido ou modificado durante o empréstimo mutável.

### Desreferenciando

* Usando referências, podemos definir o valor do proprietário usando o operador `*`; 
* Também há a possibilidade de obter um cópia do valor de propriedade usando o mesmo operador; 

### Passando dados emprestados

* O Rust permite que haja apenas uma referência mutável **ou** múltiplas referências não-mutáveis, **mas não ambas**; 
* Uma referência nunca deve viver mais do que o seu proprietário.

### Referências de Referências

* Referências podem ser usadas até mesmo em pedaços de referências.

### Tempo de vida explícito

* Funções podem ser explícitas parametrizando a assinatura da função com símbolos que ajudam a identificar quais parâmetros e valores de retorno compartilham o mesmo tempo de vida; 
* Os especificadores de tempo de vida sempre começam com um `'` (Ex: `'a`, `'b`, `'c`).

### Tempo de Vida Múltiplos

* Os especificadores de tempo de vida nos permitem sermos explícitos em certos cenários que o compilador não pode resolver sozinho; 
* Podemos distinguir todos os tempos de vida dos componentes na assinatura da função.

### Tempo de Vida Estático

* Uma variável estática é um recurso de memória criado em tempo de compilação que existe do início ao fim dentro de um programa; 
* Devem ser especificados explicitamente; 
* Em outras palavras, esse tipo de recurso dura indefinidamente até o término do programa; 
* Para especificá-lo, usamos `'static`; 
* Se algum recurso com tempo de vida estático contiver referência, todos deverão ser `'static` (nada menos do que isso viveria tempo suficiente).

### Tempo de Vida em Tipos de Dados

* Da mesma forma que as funções, as structs podem ser parametrizados com especificadores de tempo de vida; 
* Rust valida a estrutura de dados que contém as referências para que nunca dure mais do que os proprietários para os quais as suas referências apontam.

### Referências

* (https://doc.rust-lang.org/std/str/struct.Split.html)[https://doc.rust-lang.org/std/str/struct.Split.html]
* [https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
* [https://stackoverflow.com/questions/34363984/what-is-vec](https://stackoverflow.com/questions/34363984/what-is-vec)
* [https://stackoverflow.com/questions/59745873/iterator-collect-issue-with-value-of-type-vecstring-cannot-be-built-from-it](https://stackoverflow.com/questions/59745873/iterator-collect-issue-with-value-of-type-vecstring-cannot-be-built-from-it)

## Dia 12

### Literais string

* Literais string são sempre Unicode; 
* O tipo de literais é `&'static str`; 
* `&` significa que está se referindo a uma posição na memória; 
* a falta de um `&mut` significa que o compilador não permitirá modificações; 
* `'static` significa que os dados da string estarão disponíveis até o final da execução do programa; 
* `str` significa que aponta para uma sequência de bytes utf-8; 

### Caractere de escape

* Rust suporta códigos de escape comuns em linguagens baseadas em C:
* `\n` - newline; 
* `\r` - carriage return; 
* `\\` - backslash; 
* `\0` - null; 
* `\'` - single-quote; 
* Lista completa: [https://doc.rust-lang.org/reference/tokens.html](https://doc.rust-lang.org/reference/tokens.html).

### Literais de string multilinhas

* Por padrão, as strings no Rust são multilinhas; 
* Use um `\` no final da linha se você não quiser uma quebra de linha.

### Literais de string brutas

* Se você não quiser usar caracteres de escape e criar uma "string bruta", você pode utilizar a sintaxe `r#""#`.

``` rust
let a: &'static str = r#"
    <div static="conselho">
        Strings brutas são úteis para algumas situações.
    </div>
"#;
```

### Literais de string de arquivos

* Se você possui um texto muito grande, considere usar a macro `include_str!`; 

``` rust
let hello_html = include_str!("hello.html);
```

### Fatia de string

* Uma fatia de string é uma referência a uma sequência de bytes na memória que de ser um utf-8 válido; 
* uma subfatia também necessariamente deve ser um utf-8 válido; 
* `len` obtém o comprimento da string literal em bytes (não o número de caracteres); 
* `starts_with` / `ends_with` para testes básicos; 
* `is_empty` retorna true se o comprimento for zero.
* `find` retorna um `Option<usize>` da primeira posição de um texto.

### Caracteres

* Com tanta dificuldade para trabalhar com Unicode, o Rust oferece uma maneira de recuperar uma sequência de bytes utf-8 como yn vetor de caracteres do tipo `char`; 
* Um `char` sempre tem 4 bytes de comprimento (permitindo uma busca eficiente de caracteres individuais).

### Strings

* Uma `String` é uma estrutura que contém uma sequência de bytes utf-8 na memória heap; 
* Como sua memória está na pilha, ela pode ser estendida, modificada, etc, de modo que literais de strings não podem; 
* `push_str` adiciona mais bytes utf-8 ao final de uma string; 
* `replace` substitui sequências de bytes utf-8 por outras; 
* `to_lowercase` / `to_uppercase` para alterações de maiúsculas e minúsculas; 
* `trim` para cortar espaços; 
* Quando uma `String` é descartada, sua memória heap também é descartada.

### Texto com parâmetros de função

* Tanto `Strings` quando literais de strings são passados como uma fatia para funções; 

``` rust
fn gritar(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}

gritar("olá");
gritar(&String::from("adeus"));
```

### Criando strings

* O `concat` e o `join` são duas maneiras simples de criar strings.

``` rust
let olamundo = ["olá", " ", "mundo", "!"].concat();
let abc = ["a", "b", "c"].join(",");
```

### Formatando strings

* A macro `format!` nos permite criar uma string definindo uma string parametrizada com espaços reservados para onde e como os valores devem ser colocados (Ex: `{}`); 
* O `format!` usa as mesmas strings que `println!`; 
* Você pode ler mais sobre esse tipo de recurso em [https://doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/).

### Converter strings

* Muitos tipos podem ser convertidos em uma string usando o `to_string`; 
* A função genérica `parse` é usada para converter strings ou literais de strings em um valor digitado. Esta função retorna um `Result` porque pode falhar (Ex: `"123".parse::<i32>()?; `).

## Dia 13

### Encapsulamento de Dados

- Uma das principais caracteristicas da POO é o "encapsulamento";
- Rust suporta o conceito de "objeto", que é uma estrutura associada a algumas funções;
- As chamadas de métodos é bastante parecida com linguagens como Python e JS;
- O primeiro parâmetro de qualquer método é sempre uma referência à instância associada à chamada do método (`instanceOfObj.foo()`);
- Temos algumas particularidades com o "self" no Rust: `&self` referência imutável da instância, `&mut self` referência mutável da instância;
- Todos os métodos são definidos dentro de um bloco de implementação com a palavra-chave `impl`;

```rust
impl MinhaStruct {
    ...
    fn foo(&self) {
        ...
    }
}
```

### Abstração com exposição seletiva

- Rust implementa um esquema bastante comum de "privacidade" de métodos/propriedades;
- Por padrão, os campos dos métodos são acessíveis apenas ao módulo ao qual pertencem;
- A palavra-chave `pub` pode ser utilizada em campos e métodos de uma struct para expô-los fora da visibilidade do módulo.

### Polimorfismo

- Rust suporta o polimorfismo com traits;
- As traits nos permitem associar um conjunto de métodos a um tipo struct;
- Quando uma struct implementa uma trait, ela estabelece um "contrato" que nos permite interagir indiretamente com a struct por meio tipo da trait;
- Os métodos da trait implementados na struct são definidos dentro de um bloco de implementação.

```rust
trait MinhaTrait {
    fn foo(&self);
    ...
}

impl MinhaTrait for MinhaStruct {
    fn foo(&self) {
        ...
    }
    ...
}
```

### Métodos implementados nas traits

- As traits também podem ter métodos implementados;
- As funções não tem acesso direto aos campos internos de uma struct, mas podem ser úteis para compartilhar comportamentos entre muitos implementadores de traits;

### Herença de traits

- As traits podem herdar métodos de outras traits.

### Dispatch dinâmico X estático

- Métodos são executados de duas maneiras diferentes;
- Quando o tipo da instância é conhecido, temos conhecimento direto de qual função chamar (`static dispatch`);
- Quando o tipo da instância não é conhecido precisamos descobrir uma maneira de chamar a função correta (`dynamic dispatch`);
- os tipos de trait `&dyn MinhaTrait` nos dá a habilidade de trabalharmos com instâncias de objetos indiretamente usando o dynamic dispatch;
- Quando um dynamic dispatch é usado, o Rust irá encorajar a colocar o `dyn` and do seu tipo trait.

### Objetos trait

- Quando passamos uma instância de um objeto para um parâmetro do tipo `&dyn MinhaTrait`, passamos o que é chamado de "objeto trait";
- Um objeto trait nos permite chamar indiretamente os métodos corretos de uma instância.

### Manipulando dados não dimensionados

- As traits introduzem um desafio interessante quando queremos armazená-las em outra struct;
- As traits obscurecem a struct original, portanto, tanbém obscurecem o tamanho original;
- Os valores não dimensionados armazenados em structs são tratados de duas maneiras;
- Usando tipos parametrizados cria efetivamente tipos conhecidos de structs/funções, dito isso, tamanhos conhecidos (`generics`);
- Colocando instâncias no heap fornece um contorno que permite que não nos preocupemos com o tamanho do tipo atual e apenas armazenemos um ponteiro nele (`indirection`);

### Funções Genéricas

- Os genéricos no Rust trabalham lado a lado com as traits;
- Quando descrevemos um tipo parametrizado `T`, podemos restringir quais tipos pode sem usados como argumento listando as traits necessárias que o argumento deve implementar;
- Usando genéricos criamos funções tipadas estáticas em tempo de compilação que terão tipos e tamanhos conhecidos.

```rust
fn minha_funcao<T>(foo: T)
where
    T:Foo
{
    ...
}
```

### Funções Genéricas Abreviadas

- Podemos abreviar a sintaxe de função genérica.

```rust
fn main_funcao(foo: impl Foo) {
    ...
}
```

### Box

- `Box` é uma estrutura de dados que permite mover nossos dados da stack para a heap;
- `Box` é uma estrutura conhecida como "ponteiro inteligente" que contém o ponteiro dos nossos dados na heap;
- Ele frequentemente é usado como maneira de armazenar uma referência a algo em uma struct que deve saber o tamanho de seus campos;
- `Box` é uma struct com tamanho fixo (porque contém apenas um ponteiro);

```rust
Box::new(Foo { ... })
```

### Structs Genéricas Revisitada

- structs genéricas também podem ter seus tipos parametrizados restritos por traits

```rust
struct MinhaStruct<T>
where
    T: MinhaTrait
{
    foo: T
    ...
}

impl<T> MinhaStruct<T> {
    ...
}
```

### Referência

- (https://www.youtube.com/watch?v=pTB0EiLXUC8)[https://www.youtube.com/watch?v=pTB0EiLXUC8]
- (https://dpc.pw/the-faster-you-unlearn-oop-the-better-for-you-and-your-software)[https://dpc.pw/the-faster-you-unlearn-oop-the-better-for-you-and-your-software]