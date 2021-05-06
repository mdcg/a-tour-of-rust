// As literais de string geralmente são passados como uma fatia
// de string para as funções. Isso oferece muita flexibilidade para a
// maioria dos cenários em que realmente não precisa passar a propriedade.

fn gritar(msg:&str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}

fn main() {
    // gritar pode pegar emprestado um &'static str como &str
    gritar("olá");
    // gritar também pode pegar emprestado uma string como &str
    gritar(&String::from("adeus"));
}