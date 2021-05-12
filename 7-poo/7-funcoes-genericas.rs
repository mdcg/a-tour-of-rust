// Os genéricos no Rust trabalham lado a lado com as traits. Quando
// descrevemos um tipo parametrizado T, podemos restringir quais
// tipos podem ser usados como argumento listando as traits
// necessárias que o argumento deve implementar.

// Neste exemplo o tipo T deve implementar a trait Foo:

// fn minha_funcao<T>(foo: T)
// where 
//     T: Foo,
// {
//     // conhecemos o tipo real em tempo de compilação
//     ...
// }

// Usando genéricos criamos funções tipadas estáticas em tempo de
// compilaçao que terão tipos e tamanhos conhecidos, permitindo
// executar dispatchs estáticos e armazená-lo como um valor
// dimensionado.

struct CriaturaMarinha {
    pub nome: String,
    barulho: String,
}

impl CriaturaMarinha {
    pub fn pega_barulho(&self) -> &str {
        &self.barulho
    }
}

trait FazedorBarulho {
    fn faz_barulho(&self);
}

impl FazedorBarulho for CriaturaMarinha {
    fn faz_barulho(&self) {
        println!("{}", &self.pega_barulho());
    }
}

fn generic_faz_barulho<T>(criatura: &T)
where
    T: FazedorBarulho,
{
    // conhecemos o tipo real em tempo de compilação
    criatura.faz_barulho();
}

fn main() {
    let criatura = CriaturaMarinha {
        nome: String::from("Ferris"),
        barulho: String::from("glub"),
    };
    generic_faz_barulho(&criatura);
}