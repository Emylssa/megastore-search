pub fn limpar_termos(texto: &str) -> Vec<String> {
    let stopwords = ["de", "a", "e", "o", "em", "um", "para", "com"];
    texto.to_lowercase()
        .split_whitespace()
        .filter(|t| !stopwords.contains(t))
        .map(|s| s.to_string())
        .collect()
}
