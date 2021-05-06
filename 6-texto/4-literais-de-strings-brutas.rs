// As strings nos permitem escrever uma sequência de caracteres
// literais começando com r#" e terminando com "#. Ela nos
// permite inserir caracteres que, de outra forma, poderiam confundir 
// uma string normal com literais (como aspas duplas e barras
// invertidas).
fn main() {
    let a: &static str = r#"
    <div class="conselho">
        Strings brutas são úteis para algumas situações.
    </div>
    "#;
}