use megastore_search::models::*;
use megastore_search::index::*;

#[test]
fn test_indexador_basico() {
    let mut indexador = Indexador::novo();
    let produto = Produto {
        id: 10,
        nome: "Smart TV LG".to_string(),
        marca: "LG".to_string(),
        categoria: "Eletrônicos".to_string(),
    };

    indexador.indexar(&produto);
    let resultado = indexador.buscar("lg");

    assert!(resultado.is_some());
    assert!(resultado.unwrap().contains(&10));
}
