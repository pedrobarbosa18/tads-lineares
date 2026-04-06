// Exercício 10 — Simulador de fila de banco
//
// Complexidade:
//   - Chegada de cliente (enqueue): O(1) com VecDeque
//   - Atendimento (dequeue):        O(1) com VecDeque
//   - Simulação total:              O(n), onde n é o número de clientes
//
// Simulamos um banco com chegadas e atendimentos em "instantes de tempo".
// O tempo de espera de cada cliente é: (instante_de_atendimento - instante_de_chegada).

use std::collections::VecDeque;

struct Cliente {
    id: usize,
    chegada: u32,    // instante em que entrou na fila
    atendimento: u32, // tempo necessário para atender
}

fn simular_banco(clientes: Vec<(u32, u32)>) {
    // clientes: lista de (instante_chegada, tempo_atendimento)
    let mut fila: VecDeque<Cliente> = VecDeque::new();
    let mut tempo_atual = 0u32;
    let mut total_espera = 0u32;
    let mut i = 0; // índice no vetor de chegadas

    let n = clientes.len();

    println!("{:<8} {:<10} {:<12} {:<10}", "Cliente", "Chegada", "Início Aten.", "Espera");
    println!("{}", "-".repeat(45));

    while i < n || !fila.is_empty() {
        // Adiciona à fila todos que chegaram até o tempo atual
        while i < n && clientes[i].0 <= tempo_atual {
            fila.push_back(Cliente {
                id: i + 1,
                chegada: clientes[i].0,
                atendimento: clientes[i].1,
            });
            i += 1;
        }

        if let Some(cliente) = fila.pop_front() {
            // O atendimento começa no máximo entre agora e a chegada do cliente
            let inicio = tempo_atual.max(cliente.chegada);
            let espera = inicio - cliente.chegada;
            total_espera += espera;
            tempo_atual = inicio + cliente.atendimento;

            println!(
                "{:<8} {:<10} {:<12} {:<10}",
                cliente.id, cliente.chegada, inicio, espera
            );
        } else {
            // Nenhum cliente na fila — avança o tempo até o próximo
            if i < n {
                tempo_atual = clientes[i].0;
            }
        }
    }

    println!("{}", "-".repeat(45));
    println!(
        "Tempo médio de espera: {:.1} unidades",
        total_espera as f64 / n as f64
    );
}

fn main() {
    // (instante_chegada, tempo_atendimento)
    let clientes = vec![
        (0, 4),
        (2, 3),
        (4, 2),
        (7, 5),
        (10, 1),
    ];

    println!("=== Simulação de Fila de Banco ===\n");
    simular_banco(clientes);
}
