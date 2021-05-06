// Uma variável estática é um recurso de memória criado em tempo de
// compilação que existe do início ao fim dentro de um programa. ELes
// devem ter seus tipos especificados explicitamente.

// Um recurso com tempo de vida estático é um recurso de memória
// que dura indefinidamente até o término de um programa. Observe
// que, por essa definição, alguns recursos com tempo de vida estático
// podem ser criados em tempo de execução.

// Recursos com tempo de vida estático têm o especificador de tempo de
// vida especial 'static.

// Recursos 'static nunca serão descartados.

// Se os recursos com tempo de vida estático contiverem referências,
// todos deverão ser 'static (nada menos do que isso viveria tempo
// suficiente).

// Detalhes da memória:

//      - A modificação de variáveis estáticas é inerentemente perigosa,
//      porque elas são acessíveis globalmente para serem lidas por 
//      qualquer um introduzindo a possibilidade de um data race.
//      - O Rust permite o uso do bloco unsafe {...} para executar
//      algumas operações sobre as quais o compilador não pode 
//      garantir a integridade do que está guardado na memória.

static PI: f64 = 3.1415;

fn main() {
    // variáveis estáticas também podem ter escopo definido para uma função
    static mut SECRETO: &'static str = "senha";

    // strings tem o mesmo tempo de vida que as variáveis estáticas
    let msg: &'static str = "Olá Mundo!";
    let p: &static f64 = &PI;
    println!("{} {}", msg, p);

    // Você pode quebrar algumas regras, mas deve ser explícito
    unsafe {
        // podemos atribuir uma string a SECRETO porque ela também é estática
        SECRETO = "abracadabra";
        println!("{}", SECRETO);
    }
}