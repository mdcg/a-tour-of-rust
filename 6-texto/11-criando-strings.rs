// O concat e o join são duas maneiras simples, mas poderosas, de
// criar strings.
fn main() {
    let olamundo = ["olá", " ", "mundo", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", olamundo);
    println!("{}", abc);
}