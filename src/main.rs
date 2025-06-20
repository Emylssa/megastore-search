mod models;
mod index;
mod preprocess;
mod cache;

use models::Produto;
use index::Indexador;
use cache::CacheBusca;
use std::io;

fn main() {
    let mut indexador = Indexador::novo();
    let mut cache = CacheBusca::novo();

    let produtos = vec![
        Produto {
            id: 1,
            nome: "Celular Galaxy S21".to_string(),
            marca: "Samsung".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
        Produto {
            id: 2,
            nome: "Notebook Dell Inspiron".to_string(),
            marca: "Dell".to_string(),
            categoria: "Informática".to_string(),
        },
        Produto {
            id: 3,
            nome: "Tênis Nike Air".to_string(),
            marca: "Nike".to_string(),
            categoria: "Calçados".to_string(),
        },
    ];

    for produto in &produtos {
        indexador.indexar(produto);
    }

    loop {
        println!("Digite um termo de busca (ou 'sair'):");
        let mut termo = String::new();
        io::stdin().read_line(&mut termo).expect("Erro na leitura");
        let termo = termo.trim().to_lowercase();

        if termo == "sair" {
            break;
        }

        if let Some(resultados) = cache.obter(&termo) {
            println!("[CACHE] Produtos encontrados: {:?}", resultados);
            continue;
        }

        match indexador.buscar(&termo) {
            Some(resultados) => {
                println!("Produtos encontrados: {:?}", resultados);
                cache.armazenar(&termo, resultados.clone());
            }
            None => println!("Nenhum produto encontrado."),
        }
    }
}
