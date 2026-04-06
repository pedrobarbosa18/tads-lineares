// Exercício 09 — Pilha com mínimo em O(1)
//
// Complexidade:
//   - push(x): O(1) — push nas duas pilhas
//   - pop():   O(1) — pop nas duas pilhas
//   - min():   O(1) — só olha o topo da pilha auxiliar
//   - peek():  O(1) — só olha o topo da pilha principal
//
// Truque: mantemos uma pilha auxiliar de mínimos.
// Ao empilhar x, o novo mínimo é min(x, mínimo_atual).
// A pilha auxiliar sempre tem no topo "o menor valor visto até agora".
// Assim o min() é só um peek no topo da pilha auxiliar — instantâneo.

struct StackMin {
    dados: Vec<i32>,
    minimos: Vec<i32>, // topo = menor elemento atual da pilha principal
}

impl StackMin {
    fn new() -> Self {
        StackMin {
            dados: Vec::new(),
            minimos: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.dados.push(x);
        // O novo mínimo é o menor entre x e o mínimo atual
        let novo_min = match self.minimos.last() {
            Some(&atual_min) => atual_min.min(x),
            None => x, // pilha estava vazia, x é o único elemento
        };
        self.minimos.push(novo_min);
    }

    fn pop(&mut self) -> Option<i32> {
        self.minimos.pop(); // remove o mínimo associado ao elemento que sai
        self.dados.pop()
    }

    fn min(&self) -> Option<i32> {
        // O mínimo atual é sempre o topo da pilha auxiliar — O(1)
        self.minimos.last().copied()
    }

    fn peek(&self) -> Option<i32> {
        self.dados.last().copied()
    }

    fn is_empty(&self) -> bool {
        self.dados.is_empty()
    }
}

fn main() {
    let mut pilha = StackMin::new();

    let valores = vec![5, 3, 7, 2, 8, 1, 4];
    println!("Empilhando: {:?}", valores);

    for v in valores {
        pilha.push(v);
        println!("  push({}) | topo={:?} | min={:?}", v, pilha.peek(), pilha.min());
    }

    println!("\nDesempilhando:");
    while !pilha.is_empty() {
        let removido = pilha.pop();
        println!("  pop()={:?} | min agora={:?}", removido, pilha.min());
    }
}
