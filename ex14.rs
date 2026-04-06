// Exercício 11 — Impressora compartilhada
//
// Complexidade:
//   - Adicionar trabalho (enqueue): O(1) com VecDeque
//   - Processar trabalho (dequeue): O(1) com VecDeque
//   - Processar fila completa:      O(n), onde n é o número de trabalhos
//
// FIFO puro: quem chegou primeiro, imprime primeiro.
// Cada trabalho tem nome e número de páginas — reportamos cada um ao processar.

use std::collections::VecDeque;

struct Trabalho {
    nome: String,
    paginas: u32,
}

struct Impressora {
    fila: VecDeque<Trabalho>,
}

impl Impressora {
    fn new() -> Self {
        Impressora {
            fila: VecDeque::new(),
        }
    }

    fn adicionar(&mut self, nome: &str, paginas: u32) {
        self.fila.push_back(Trabalho {
            nome: nome.to_string(),
            paginas,
        });
        println!("  [+] Trabalho adicionado: \"{}\" ({} pág.)", nome, paginas);
    }

    fn processar_todos(&mut self) {
        println!("\n  Iniciando impressão...");
        let mut total_paginas = 0u32;

        while let Some(trabalho) = self.fila.pop_front() {
            println!(
                "  Imprimindo: \"{}\" — {} página(s)",
                trabalho.nome, trabalho.paginas
            );
            total_paginas += trabalho.paginas;
        }

        println!("  Fila vazia. Total impresso: {} páginas.", total_paginas);
    }

    fn tamanho_fila(&self) -> usize {
        self.fila.len()
    }
}

fn main() {
    let mut impressora = Impressora::new();

    println!("=== Impressora Compartilhada ===\n");
    println!("Adicionando trabalhos:");
    impressora.adicionar("Relatório Mensal", 12);
    impressora.adicionar("Apresentação TCC", 35);
    impressora.adicionar("Receita de Bolo", 1);
    impressora.adicionar("Manual do Usuário", 80);
    impressora.adicionar("Nota Fiscal", 2);

    println!("\n  Trabalhos na fila: {}", impressora.tamanho_fila());
    impressora.processar_todos();
}
