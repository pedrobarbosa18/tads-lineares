// Exercício 15 — Máximo de janela deslizante com VecDeque
//
// Complexidade:
//   - Solução ingênua (max por janela): O(n * k)
//     Para cada posição recalcula o max da janela — caro para k grande.
//
//   - Solução com deque monotônico: O(n)
//     Cada elemento entra e sai do deque no máximo uma vez.
//     O deque mantém índices em ordem decrescente de valor.
//     O topo do deque é SEMPRE o índice do máximo da janela atual.
//
// Intuição: o deque é como uma fila de candidatos a "melhor da turma".
// Ao entrar um valor maior, elimina os menores que nunca serão o máximo.

use std::collections::VecDeque;

fn janela_maxima(vec: &[i32], k: usize) -> Vec<i32> {
    let n = vec.len();
    if k == 0 || n == 0 {
        return vec![];
    }

    let mut resultado = Vec::with_capacity(n - k + 1);
    // Deque guarda ÍNDICES dos elementos candidatos ao máximo
    // Invariante: vec[deque[0]] é sempre o maior da janela atual
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove índices que saíram da janela (mais antigos que i - k)
        while let Some(&frente) = deque.front() {
            if frente + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        // Remove do fundo elementos menores que o atual (nunca serão o max)
        while let Some(&fundo) = deque.back() {
            if vec[fundo] <= vec[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        // Janela completa: o máximo é sempre o índice na frente do deque
        if i >= k - 1 {
            resultado.push(vec[*deque.front().unwrap()]);
        }
    }

    resultado
}

fn main() {
    let casos = vec![
        (vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        (vec![1, 2, 3, 4, 5], 2),
        (vec![5, 4, 3, 2, 1], 3),
        (vec![1], 1),
    ];

    println!("=== Máximo de Janela Deslizante ===\n");
    for (vec, k) in casos {
        let maximos = janela_maxima(&vec, k);
        println!("  Vec: {:?}  k={}", vec, k);
        println!("  Máximos: {:?}\n", maximos);
    }
}
