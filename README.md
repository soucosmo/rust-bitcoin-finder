# Rust Bitcoin Finder

Ferramenta para encontrar bitcoins escrita em Rust. Esta ferramenta é especialmente desenvolvida para resolver desafios de carteiras como os encontrados em [Bitcoin Puzzle Transactions](https://privatekeys.pw/puzzles/bitcoin-puzzle-tx).

## Funcionalidades

- Converte chaves privadas em chaves públicas utilizando a curva elíptica secp256k1.
- Verifica se as chaves públicas correspondem a endereços de carteira de bitcoin conhecidos.
- Utiliza processamento paralelo para otimizar a busca.

## Requisitos

- Rust (versão 1.54 ou superior)

## Instalação

1. **Instalando Rust**

   Rust pode ser instalado utilizando o instalador `rustup`. Siga as instruções abaixo:

   No Linux e macOS:
   ```bash
   sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
   

   No Windows:
   Baixe e execute o instalador do Rust disponível em [rustup.rs](https://rustup.rs/).

2. **Verificando a instalação do Rust**

   Após a instalação, verifique se o Rust foi instalado corretamente:
   sh

   ```bash
   rustc --version
   ```
   

3. **Inicializando o projeto e instalando dependências**

   Clone o repositório e navegue até o diretório do projeto:
   ```bash
   sh
   git clone https://github.com/soucosmo/rust-bitcoin-finder
   cd rust-bitcoin-finder
   ```
   

   Compile e rode o projeto e baixe as dependências:
   ```bash
   sh
   cargo run --release
   ```

Agora você está pronto para executar a ferramenta Rust Bitcoin Finder.

### Gostou e quer ajudar? Nos envie bitcoin ou dogecoin.
| Criptomoeda   | Endereço                            |
| ---           | ----------------------------------- |
| Bitcoin       | 3HJECiexkY2VZuJvQ83eoyb1rjE6Vx1yP9  |
| Dogecoin      | DD3aGVrzfNDMkeE2Gc7rHLjbmdwMV4DBqG  |
