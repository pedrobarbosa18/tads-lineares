// Exercício 04 — Mescla ordenada de dois Vec<i32> já ordenados
//
// Complexidade:
//   - Mescla manual (merge): O(n + m)
//     Dois ponteiros caminham pelos dois vetores uma vez cada.
//     É como intercalar duas filas ordenadas de pessoas — sempre pega o menor da frente.
//   - extend + sort: O((n+m) log(n+m))
//     Mais simples de escrever, porém reordena tudo do zero — menos eficiente.

fn mesclar_manual(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    // Caminha pelos dois vetores simultaneamente, pegando sempre o menor
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultado.push(a[i]);
            i += 1;
        } else {
            resultado.push(b[j]);
            j += 1;
        }
    }

    // Copia os restos (um dos vetores pode ter sobrado)
    resultado.extend_from_slice(&a[i..]);
    resultado.extend_from_slice(&b[j..]);

    resultado
}

fn mesclar_extend_sort(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::new();
    resultado.extend_from_slice(a);
    resultado.extend_from_slice(b);
    resultado.sort(); // O((n+m) log(n+m))
    resultado
}

fn main() {
    let a = vec![1, 3, 5, 7, 9];
    let b = vec![2, 4, 6, 8, 10];

    println!("Vec A: {:?}", a);
    println!("Vec B: {:?}", b);

    let manual = mesclar_manual(&a, &b);
    println!("Mescla manual O(n+m):          {:?}", manual);

    let com_sort = mesclar_extend_sort(&a, &b);
    println!("Mescla extend+sort O(n log n): {:?}", com_sort);
}
