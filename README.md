# Sistema de Busca Otimizado para Catálogo de Produtos – MegaStore

## 🧾 Descrição do Projeto

Este projeto tem como objetivo o desenvolvimento de um sistema de busca eficiente, escalável e seguro para um catálogo de e-commerce com milhões de produtos, representado pela empresa fictícia MegaStore. A aplicação foi desenvolvida em Rust e utiliza estruturas de dados otimizadas como **tabelas hash**, técnicas de **pré-processamento textual** e **cache** para garantir rapidez e relevância nas buscas.

## 🛠 Tecnologias Utilizadas

- Linguagem: **Rust** (edition 2021)
- Crates (bibliotecas):
  - `std::collections::HashMap`: para indexação e cache
  - `lazy_static`: para simulação de banco de dados em memória (opcional)
  - `regex`: para normalização e limpeza de texto (pré-processamento)
- Ferramentas:
  - `cargo`: Gerenciador de pacotes e build do Rust
  - `cargo test`: Execução de testes automatizados

## 🚀 Como Executar o Sistema de Busca

### Pré-requisitos:
- Ter o Rust instalado: https://www.rust-lang.org/tools/install

### Comandos:

```bash
# Clone o repositório
git clone https://github.com/Emylssa/megastore-search.git
cd megastore-search

# Execute o projeto
cargo run
Você verá no terminal:
Digite um termo de busca (ou 'sair'):

# Digite um termo como:
nike
notebook
informática
samsung

Para sair, digite: sair

## Como Executar os Testes 🧪
cargo test

📝 Exemplos de Uso
Produto Simulado:

Produto {
    id: 3,
    nome: "Tênis Nike Air",
    marca: "Nike",
    categoria: "Calçados",
}

## Resultado Esperado:
Produtos encontrados: [3]

Busca repetida:
[CACHE] Produtos encontrados: [3]

🧩 Arquitetura do Sistema

megastore-search/
├── Cargo.toml
├── src/
│   ├── main.rs         # Função principal e execução do loop de busca
│   ├── models.rs       # Definição da struct Produto
│   ├── index.rs        # Indexação de produtos com HashMap
│   ├── preprocess.rs   # Pré-processamento de texto (limpeza e normalização)
│   └── cache.rs        # Sistema de cache para buscas repetidas
├── tests/
│   └── integration.rs  # Testes de integração e validação de funcionalidades



### 📚 Algoritmos e Estruturas de Dados Utilizados
Tabelas Hash (HashMap):

Usadas para indexar rapidamente os produtos com base nos termos de busca.
O acesso é O(1) em média, garantindo performance mesmo com grandes volumes de dados.

#Busca por palavras-chave com pré-processamento:
Eliminação de stop words, conversão para minúsculas, remoção de pontuações.

#Sistema de Cache:
Armazena os resultados de buscas recentes para evitar reprocessamento.

###⚙️ Considerações sobre Desempenho e Escalabilidade

Desempenho:
As buscas são feitas em tempo constante (O(1)) devido ao uso de HashMap.
A reutilização via cache reduz chamadas repetidas e melhora tempo de resposta.

#Escalabilidade:
O sistema é modular e pode ser expandido para funcionar com:
Banco de dados real
APIs externas
Frameworks web (como Actix-web)

#Testes

Simulações foram feitas com dezenas de produtos e múltiplas buscas.
Tempo de resposta foi instantâneo mesmo com simulação de 1000+ entradas.




🚨 Este projeto é fictício e criado para fins educacionais como parte do curso de Análise e Desenvolvimento de Sistemas no Centro Universitário UniFECAF.
Uso livre para fins de aprendizagem.
