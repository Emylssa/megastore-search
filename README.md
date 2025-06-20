# Sistema de Busca Otimizado para Catálogo de Produtos – MegaStore

Este projeto simula um sistema de busca eficiente para um e-commerce com milhões de produtos, utilizando a linguagem **Rust** e técnicas de otimização como **tabelas hash**, **pré-processamento de texto** e **cache**.

## 🎯 Objetivo

Desenvolver um sistema de busca eficiente, escalável e seguro para o catálogo da MegaStore, otimizando o tempo de resposta e a precisão dos resultados.

## 📦 Tecnologias Utilizadas

- **Rust** (Edition 2021)
- `std::collections::HashMap`
- Terminal interativo via `cargo run`

## 📁 Estrutura do Projeto

megastore-search/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── models.rs
│   ├── index.rs
│   ├── preprocess.rs
│   └── cache.rs
├── tests/
│   └── integration.rs

## 🚀 Como Executar o Projeto

1. Clone o repositório:
   ```bash
   git clone https://github.com/Emylssa/megastore-search.git
   cd megastore-search

 ##  Execute o sistema:
- digite: cargo run

## Digite um termo de busca como:
nike
informática
tênis

## Para sair do programa:
sair

## executar os Testes
Para verificar se tudo está funcionando corretamente:
cargo test

## Produto simulado:

Produto {
    id: 3,
    nome: "Tênis Nike Air",
    marca: "Nike",
    categoria: "Calçados",
}

## Busca:
Digite um termo de busca (ou 'sair'): nike

## Saída esperada:
Produtos encontrados: [3]

## Busca repetida:
[CACHE] Produtos encontrados: [3]



🪪 Licença
Este projeto é fictício e acadêmico, criado para fins educacionais no curso de Análise e Desenvolvimento de Sistemas do Centro Universitário UniFECAF.

