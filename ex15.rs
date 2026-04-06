// Exercício 12 — Buffer circular com overwrite (FilaCircular)
//
// Complexidade:
//   - enqueue (adicionar mensagem): O(1)
//     Se cheio, descarta a mais antiga (pop_front) antes de inserir — ainda O(1).
//   - dequeue (consumir mensagem):  O(1)
//   - peek (ver próxima):           O(1)
//
// É como uma fita de gravação que começa a sobrescrever do início quando cheia.
// Útil para logs de tamanho fixo, buffers de sensores, janelas deslizantes.

use std::collections::VecDeque;

struct FilaCircular {
    buffer: VecDeque<String>,
    capacidade: usize,
    descartados: usize, // contador de mensagens sobrescritas
}

impl FilaCircular {
    fn new(capacidade: usize) -> Self {
        FilaCircular {
            buffer: VecDeque::with_capacity(capacidade),
            capacidade,
            descartados: 0,
        }
    }

    fn enqueue(&mut self, mensagem: &str) {
        if self.buffer.len() == self.capacidade {
            // Buffer cheio: descarta a mensagem mais antiga (frente da fila)
            let descartada = self.buffer.pop_front().unwrap();
            self.descartados += 1;
            println!("  [OVERWRITE] Descartou: \"{}\"", descartada);
        }
        self.buffer.push_back(mensagem.to_string());
        println!("  [+] Adicionou: \"{}\" | buffer: {}/{}", mensagem, self.buffer.len(), self.capacidade);
    }

    fn dequeue(&mut self) -> Option<String> {
        self.buffer.pop_front()
    }

    fn peek(&self) -> Option<&str> {
        self.buffer.front().map(|s| s.as_str())
    }

    fn esta_cheio(&self) -> bool {
        self.buffer.len() == self.capacidade
    }

    fn tamanho(&self) -> usize {
        self.buffer.len()
    }

    fn status(&self) {
        println!("\n  Buffer atual ({}/{}): {:?}", self.tamanho(), self.capacidade, self.buffer);
        println!("  Total descartadas: {}", self.descartados);
    }
}

fn main() {
    println!("=== Buffer Circular (capacidade: 3) ===\n");
    let mut buf = FilaCircular::new(3);

    buf.enqueue("Msg 1");
    buf.enqueue("Msg 2");
    buf.enqueue("Msg 3");
    println!("  Próxima a sair: {:?}", buf.peek());
    buf.enqueue("Msg 4"); // descarta Msg 1
    buf.enqueue("Msg 5"); // descarta Msg 2

    buf.status();

    println!("\n  Consumindo mensagens:");
    while let Some(msg) = buf.dequeue() {
        println!("  Consumiu: \"{}\"", msg);
    }
}
