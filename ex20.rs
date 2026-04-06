// Exercício 17 — Comparação de desempenho: Vec ingênua vs VecDeque vs FilaCircular
//
// Complexidade por estrutura:
//
//   Vec ingênua (enqueue = push_back, dequeue = remove(0)):
//     enqueue: O(1) amortizado
//     dequeue: O(n) — todos os elementos são deslocados uma posição à esquerda
//     Total n operações: O(n²) — muito caro para filas grandes
//
//   VecDeque (deque de biblioteca padrão):
//     enqueue: O(1) amortizado (push_back)
//     dequeue: O(1) amortizado (pop_front)
//     Total: O(n) — ideal para filas
//
//   FilaCircular manual (array com head/tail):
//     enqueue: O(1) — move o ponteiro tail
//     dequeue: O(1) — move o ponteiro head
//     Total: O(n) — sem realocação, previsível em memória

use std::collections::VecDeque;
use std::time::Instant;

const N: usize = 10_000;

// --- Vec ingênua como fila ---
fn benchmark_vec_ingenua() -> u128 {
    let mut vec: Vec<i32> = Vec::new();
    let inicio = Instant::now();

    for i in 0..N as i32 {
        vec.push(i);          // enqueue: O(1) amortizado
    }
    for _ in 0..N {
        vec.remove(0);        // dequeue: O(n) — PROBLEMA!
    }

    inicio.elapsed().as_micros()
}

// --- VecDeque da std ---
fn benchmark_vecdeque() -> u128 {
    let mut deque: VecDeque<i32> = VecDeque::new();
    let inicio = Instant::now();

    for i in 0..N as i32 {
        deque.push_back(i);   // enqueue: O(1)
    }
    for _ in 0..N {
        deque.pop_front();    // dequeue: O(1)
    }

    inicio.elapsed().as_micros()
}

// --- Fila circular manual ---
struct FilaCircularBench {
    buffer: Vec<i32>,
    head: usize,
    tail: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircularBench {
    fn new(cap: usize) -> Self {
        FilaCircularBench {
            buffer: vec![0; cap],
            head: 0,
            tail: 0,
            tamanho: 0,
            capacidade: cap,
        }
    }

    fn enqueue(&mut self, val: i32) {
        self.buffer[self.tail] = val;
        self.tail = (self.tail + 1) % self.capacidade;
        self.tamanho += 1;
    }

    fn dequeue(&mut self) -> i32 {
        let val = self.buffer[self.head];
        self.head = (self.head + 1) % self.capacidade;
        self.tamanho -= 1;
        val
    }
}

fn benchmark_circular() -> u128 {
    let mut fila = FilaCircularBench::new(N);
    let inicio = Instant::now();

    for i in 0..N as i32 {
        fila.enqueue(i);      // O(1)
    }
    for _ in 0..N {
        fila.dequeue();       // O(1)
    }

    inicio.elapsed().as_micros()
}

fn main() {
    println!("=== Comparação de Desempenho (n = {}) ===\n", N);
    println!("{:<25} {:>15}", "Estrutura", "Tempo (µs)");
    println!("{}", "-".repeat(42));

    let t1 = benchmark_vec_ingenua();
    println!("{:<25} {:>15}", "Vec ingênua (O(n²))", t1);

    let t2 = benchmark_vecdeque();
    println!("{:<25} {:>15}", "VecDeque  (O(n))", t2);

    let t3 = benchmark_circular();
    println!("{:<25} {:>15}", "FilaCircular (O(n))", t3);

    println!("{}", "-".repeat(42));
    println!("\nVec ingênua é ~{:.0}x mais lenta que VecDeque", t1 as f64 / t2.max(1) as f64);
    println!("Isso confirma o custo O(n²) vs O(n) na prática.");
}
