use std::collections::HashMap;

pub struct CacheBusca {
    cache: HashMap<String, Vec<u32>>,
}

impl CacheBusca {
    pub fn novo() -> Self {
        CacheBusca { cache: HashMap::new() }
    }

    pub fn obter(&self, termo: &str) -> Option<&Vec<u32>> {
        self.cache.get(&termo.to_lowercase())
    }

    pub fn armazenar(&mut self, termo: &str, resultado: Vec<u32>) {
        self.cache.insert(termo.to_lowercase(), resultado);
    }
}
