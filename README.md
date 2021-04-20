# a-tour-of-rust

## Dia 1

### Variáveis

- Rust consegue inferir o tipo da variável em 99% dos casos;
- Você explicitamente pode especificar o tipo da variável que deseja trabalhar;
- Rust suporta shadowing;
- É uma boa prática utilizar o padrão snake_case para especificar nomes de variáveis.

### Modificando valores

- Rust trabalha com dois tipos valores: Mutáveis e Imutáveis;
- Para valores mutáveis, o compilador permitirá que a variável seja lida e gravada;
- Para valores imutáives, o compiladores permitirá apenas a leitura da variável;
- Para marcar uma variável como Mutável, utilize a palavra reservada `mut`.

### Tipos Básicos

- Booleanos: `bool` para representar verdadeiro/falso;
- Números inteiros sem sinal, para representar números inteiros não negativos: `u8`, `u32`, `u64`, `u128`;
- Números inteiros com sinal, para reprensetar números inteiros positivos e negativos: `i8`, `i32`, `i64`, `i128`;
- Ponteiros de números inteiros para representar índices e o comprimento de coleções em memória: `usize`, `isize`;
- Números de ponto flutuante: `f32`, `f64`;
- Tuplas para passar sequências de valores fixos para a pilha: `(value, value, ...)`;
- Matriz: Uma coleção de elementos similares com comprimento fixo conhecidos em tempo de compilação;
- Slices: Uma coleção de elementos similares com comprimento conhecidos em tempo de execução;
- String Slice: Texto com comprimento conhecido em tempo de execução.

### Conversão de tipos básicos

- Você deve ser específico ao utilizar valores númericos, pois, por exemplo, você não pode operar um `u8` com um `u32` sem que não aconteça nenhum erro;
- Você pode declarar variáveis númericas especificando o tipo logo após o número desejado, por exemplo: `10u8`, `10u32`, `10u64`, etc;
- Rust permite que você converta tipos numéricos utilizando a palavra reservada `as`, por exemplo: 

```rust
let a = 13u8;
let b = 7u32;
let c = a as u32 + b;
```
