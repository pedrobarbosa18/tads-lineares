// Exercício 08 — Verificação de delimitadores balanceados
//
// Complexidade:
//   - verificar(expr): O(n), onde n é o número de caracteres
//     Percorremos a string uma vez. Cada char é no máximo um push ou pop — O(1) cada.
//
// Intuição: ao encontrar abre-chave/parêntese/colchete, empilhamos.
// Ao encontrar fecha, verificamos se o topo da pilha é o par correto.
// Se não for, ou se a pilha estiver vazia ao fechar, a expressão é inválida.

fn verificar_balanceamento(expr: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();

    for c in expr.chars() {
        match c {
            // Abre: empilha esperando o fechamento correspondente
            '(' | '[' | '{' => pilha.push(c),

            // Fecha: verifica se o topo é o par correto
            ')' => {
                if pilha.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if pilha.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if pilha.pop() != Some('{') {
                    return false;
                }
            }

            // Outros caracteres ignorados
            _ => {}
        }
    }

    // Ao final, a pilha deve estar vazia — tudo que abriu foi fechado
    pilha.is_empty()
}

fn main() {
    let casos = vec![
        ("{[()]}", true),
        ("([)]", false),
        ("(((", false),
        ("", true),
        ("{[]()}", true),
        ("((()))", true),
        ("{[}", false),
        ("()[]{}", true),
    ];

    for (expr, esperado) in casos {
        let resultado = verificar_balanceamento(expr);
        let status = if resultado == esperado { "✓" } else { "✗" };
        println!("{} \"{}\" => {} (esperado: {})", status, expr, resultado, esperado);
    }
}
