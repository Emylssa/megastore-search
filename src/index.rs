use std::collections::HashMap;
use crate::models::Produto;
use crate::preprocess::limpar_termos;

pub struct Indexador {
    pub indice: HashMap<String, Vec<u32>>,
}

impl Indexador {
    pub fn novo() -> Self {
        Indexador { indice: HashMap::new() }
    }

    pub fn indexar(&mut self, produto: &Produto) {
        let texto = format!("{} {} {}", produto.nome, produto.marca, produto.categoria);
        let termos = limpar_termos(&texto);

        for termo in termos {
            self.indice.entry(termo).or_default().push(produto.id);
        }
    }

    pub fn buscar(&self, termo: &str) -> Option<&Vec<u32>> {
        self.indice.get(&termo.to_lowercase())
    }
}
