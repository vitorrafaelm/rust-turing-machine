# Máquina de Turing - a^n b^n c^n

Programa em Rust que reconhece a linguagem: `L = {a^n b^n c^n | n >= 1}`

## Instalação

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

## Execução

```bash
cd /Users/vitorrafael/Desktop/studies/ufersa/computing-theory/rust
cargo build -q --release && ./target/release/turing_machine
```

## Adicionar Teste

Edite `src/main.rs` e procure a função `main()`. Antes da última linha, adicione:

```rust
test_word("sua_palavra", true);   // true = deve aceitar
test_word("outra_palavra", false); // false = deve rejeitar
```

Depois execute novamente.

## Exemplos

Palavras que aceita:

- `abc` (1 a, 1 b, 1 c)
- `aabbcc` (2 a's, 2 b's, 2 c's)
- `aaabbbccc` (3 a's, 3 b's, 3 c's)

Palavras que rejeita:

- `ab` (faltam c's)
- `aabc` (desbalanceado)
- `aabcbc` (ordem errada)
