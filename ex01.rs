# Estruturas de Dados e Análise de Algoritmos
**Aula 06 — TADs Lineares: Vec, Pilha, Fila, Deque**

**Pedro Henrique Alves Barbosa — RA: 123222053**  
Unidade Curricular: Estruturas de Dados e Análise de Algoritmos | Código: 0006963  
Professor: Alexandre Montanha

---

## Como rodar

### Pré-requisito
Ter o [Rust instalado](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Rodar um exercício específico
```sh
cargo run --bin ex01
cargo run --bin ex05
# ... e assim por diante até ex20
```

### Rodar todos de uma vez
```sh
for i in $(seq -w 1 20); do
  echo "=== Exercício $i ===" && cargo run --bin ex$(printf "%02d" $i) --quiet
done
```

### Compilar tudo sem rodar
```sh
cargo build
```

---

## Exercícios

### Grupo 1 — Vec e Operações Básicas

| # | Descrição | Complexidade principal |
|---|-----------|----------------------|
| 01 | Inversão de Vec com push/pop | O(n) |
| 02 | Contador de ocorrências de letras | O(n) |
| 03 | Remoção de pares sem `.retain()` | O(n) novo vec · O(n²) in-place |
| 04 | Mescla ordenada de dois Vecs | O(n+m) manual · O(n log n) sort |

### Grupo 2 — Pilha (Stack)

| # | Descrição | Complexidade principal |
|---|-----------|----------------------|
| 05 | Calculadora RPN | O(n) por token |
| 06 | Histórico de navegação Voltar/Avançar | O(1) por operação |
| 07 | Editor com Desfazer/Refazer (Ctrl+Z/Y) | O(1) por operação |
| 08 | Verificação de delimitadores balanceados | O(n) |
| 09 | StackMin com `min()` em O(1) | O(1) push, pop e min |

### Grupo 3 — Fila (Queue)

| # | Descrição | Complexidade principal |
|---|-----------|----------------------|
| 10 | Simulador de fila de banco | O(n) |
| 11 | Impressora compartilhada | O(n) |
| 12 | Buffer circular com overwrite | O(1) por operação |
| 13 | Fila de prioridade manual (FIFO no empate) | O(1) enqueue · O(n) dequeue |

### Grupo 4 — Deque

| # | Descrição | Complexidade principal |
|---|-----------|----------------------|
| 14 | Palíndromo com VecDeque | O(n) |
| 15 | Máximo de janela deslizante | O(n) com deque monotônico |
| 16 | Fila de tarefas com frente prioritária | O(1) por operação |

### Grupo 5 — Reflexão e Análise

| # | Descrição | Complexidade principal |
|---|-----------|----------------------|
| 17 | Benchmark: Vec ingênua vs VecDeque vs FilaCircular | O(n²) vs O(n) vs O(n) |
| 18 | Qual TAD usar em cada cenário? | Análise teórica |
| 19 | `processar_em_lotes` com VecDeque | O(n) |
| 20 | Round Robin com fila circular | O(n + T/Q) |

---

## Resumo de complexidades

```
push / pop (Vec como pilha)   → O(1) amortizado
pop_front  (Vec ingênua)      → O(n)  ← evitar para filas grandes
push_back / pop_front VecDeque → O(1) amortizado  ← preferido para filas
min() no StackMin             → O(1)  ← truque da pilha auxiliar
Janela deslizante (deque mono)→ O(n)  ← cada elemento entra/sai 1x
Fila de prioridade linear     → O(n) dequeue ← heap seria O(log n)
```
