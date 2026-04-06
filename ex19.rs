// Exercício 16 — Fila de tarefas com prioridade de frente
//
// Complexidade:
//   - adicionar_urgente (push_front): O(1)
//   - adicionar_normal  (push_back):  O(1)
//   - processar (pop_front):          O(1)
//
// O deque resolve isso naturalmente: tarefas urgentes vão para a frente,
// normais para o fundo. Todas saem pela frente na ordem certa.
// É como uma fila com faixa preferencial: quem é urgente entra na frente.

use std::collections::VecDeque;

#[derive(Debug)]
struct Tarefa {
    descricao: String,
    urgente: bool,
}

struct FilaTarefas {
    deque: VecDeque<Tarefa>,
}

impl FilaTarefas {
    fn new() -> Self {
        FilaTarefas {
            deque: VecDeque::new(),
        }
    }

    fn adicionar_urgente(&mut self, descricao: &str) {
        // Urgente vai para a FRENTE — será processada antes das normais
        self.deque.push_front(Tarefa {
            descricao: descricao.to_string(),
            urgente: true,
        });
        println!("  [URGENTE] \"{}\" → frente da fila", descricao);
    }

    fn adicionar_normal(&mut self, descricao: &str) {
        // Normal vai para o FUNDO — processada após as urgentes
        self.deque.push_back(Tarefa {
            descricao: descricao.to_string(),
            urgente: false,
        });
        println!("  [normal]  \"{}\" → fundo da fila", descricao);
    }

    fn processar(&mut self) -> Option<Tarefa> {
        // Sempre remove da frente — urgentes sairão primeiro
        self.deque.pop_front()
    }

    fn tamanho(&self) -> usize {
        self.deque.len()
    }
}

fn main() {
    println!("=== Fila de Tarefas com Prioridade de Frente ===\n");
    let mut fila = FilaTarefas::new();

    println!("Adicionando tarefas:");
    fila.adicionar_normal("Responder e-mails");
    fila.adicionar_normal("Fazer relatório");
    fila.adicionar_urgente("Servidor caiu!");
    fila.adicionar_normal("Reunião de planning");
    fila.adicionar_urgente("Bug crítico em produção");

    println!("\nProcessando ({} tarefas):", fila.tamanho());
    let mut ordem = 1;
    while let Some(tarefa) = fila.processar() {
        let tipo = if tarefa.urgente { "🚨 URGENTE" } else { "  normal " };
        println!("  {}. [{}] \"{}\"", ordem, tipo, tarefa.descricao);
        ordem += 1;
    }
}
