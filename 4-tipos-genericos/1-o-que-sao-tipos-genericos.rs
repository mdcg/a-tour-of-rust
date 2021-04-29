// Tipos genéricos nos permitem definir parcialmente uma
// struct ou enum, deixando que o compilador instancie
// uma versão definida em tempo de compilação totalmente
// com base em nosso código.

// Geralmente o Rust pode inferir o tipo final observando
// a nossa interação, mas se ele precisar de ajuda você
// sempre pode ser explícito usando o operador ::<T>,
// também conhecido pelo nome "turbofish".

struct Sacola<T> {
    item: T,
}

fn main() {
    // Nota: Usando tipos genéricos aqui nós criamos tipos em tempo de compilação
    // fazendo nosso código ficar maior . O Turbofish nos permite sermos explícitos.
    let i32_sacola = Sacola::<i32> { item: 42 };
    let bool_sacola = Sacola::<bool> { item: true };

    // O Rust pode inferir tipos para genéricos também! 
    let float_sacola = Sacola { item: 3.14 };
    
    // Nota: nunca faça isso na vida real!
    let sacola_na_sacola = Sacola {
        item: Sacola { item: "boom!" },
    };

    println!("{} {} {} {}",
        i32_sacola.item,
        bool_sacola.item,
        float_sacola.item,
        sacola_na_sacola.item,
    );
}