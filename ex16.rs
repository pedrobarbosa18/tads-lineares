// Exercício 13 — Fila de prioridade manual (sem heap)
//
// Complexidade:
//   - enqueue(item, prioridade): O(1) — apenas push no vetor
//   - dequeue():                 O(n) — busca linear pelo maior prioritário
//     Para cada dequeue, percorremos todos os elementos para achar o maior.
//     Itens de mesma prioridade seguem FIFO (usamos contador de ordem para desempatar).
//
// Prévia de heap: com heap seria O(log n) para ambas operações.
// Aqui sacrificamos dequeue por simplicidade de implementação.

struct FilaPrioridade {
    itens: Vec<(i32, u64, String)>, // (prioridade, ordem_chegada, conteúdo)
    contador: u64,                   // garante desempate FIFO entre mesma prioridade
}

impl FilaPrioridade {
    fn new() -> Self {
        FilaPrioridade {
            itens: Vec::new(),
            contador: 0,
        }
    }

    fn enqueue(&mut self, item: &str, prioridade: i32) {
        self.itens.push((prioridade, self.contador, item.to_string()));
        self.contador += 1;
        println!("  [+] \"{}\" (prioridade {})", item, prioridade);
    }

    fn dequeue(&mut self) -> Option<String> {
        if self.itens.is_empty() {
            return None;
        }

        // Busca linear: acha o índice do item com maior prioridade e menor ordem (FIFO)
        // O(n) — percorre todos os itens
        let idx_max = self.itens
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                // Maior prioridade vence; em empate, menor ordem_chegada vence (FIFO)
                a.0.cmp(&b.0).then(b.1.cmp(&a.1))
            })
            .map(|(i, _)| i)
            .unwrap();

        let (prioridade, _, conteudo) = self.itens.remove(idx_max);
        println!("  [-] \"{}\" (prioridade {})", conteudo, prioridade);
        Some(conteudo)
    }

    fn tamanho(&self) -> usize {
        self.itens.len()
    }
}

fn main() {
    println!("=== Fila de Prioridade Manual ===\n");
    let mut fila = FilaPrioridade::new();

    println!("Adicionando itens:");
    fila.enqueue("Tarefa comum A", 1);
    fila.enqueue("Tarefa urgente", 5);
    fila.enqueue("Tarefa comum B", 1); // mesma prioridade que A — FIFO
    fila.enqueue("Tarefa média",   3);
    fila.enqueue("Tarefa crítica", 5); // mesma prioridade que urgente — FIFO

    println!("\nProcessando em ordem de prioridade (FIFO no empate):");
    while fila.tamanho() > 0 {
        fila.dequeue();
    }
}
