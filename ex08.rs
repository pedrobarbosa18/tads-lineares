// Exercício 05 — Calculadora RPN (Notação Polonesa Reversa)
//
// Complexidade:
//   - Avaliação da expressão: O(n), onde n é o número de tokens
//     Cada token é processado exatamente uma vez — push ou pop na pilha.
//     Ex: "3 4 + 2 *" → empilha 3, empilha 4, soma → 7, empilha 2, multiplica → 14

fn avaliar_rpn(expressao: &str) -> Result<f64, String> {
    // Vec<f64> usado como pilha — push/pop são O(1) amortizado
    let mut pilha: Vec<f64> = Vec::new();

    for token in expressao.split_whitespace() {
        match token {
            // Operadores: desempilha dois operandos, opera e empilha o resultado
            "+" | "-" | "*" | "/" => {
                let b = pilha.pop().ok_or("Pilha vazia ao tentar pop")?;
                let a = pilha.pop().ok_or("Pilha vazia ao tentar pop")?;
                let resultado = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("Divisão por zero".to_string());
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };
                pilha.push(resultado);
            }
            // Números: converte e empilha
            num => {
                let valor: f64 = num.parse().map_err(|_| format!("Token inválido: {}", num))?;
                pilha.push(valor);
            }
        }
    }

    // Ao final, deve restar exatamente um valor na pilha — o resultado
    if pilha.len() == 1 {
        Ok(pilha[0])
    } else {
        Err("Expressão malformada".to_string())
    }
}

fn main() {
    let expressoes = vec![
        ("3 4 + 2 *", 14.0),   // (3+4)*2
        ("5 1 2 + 4 * + 3 -", 14.0), // 5+((1+2)*4)-3
        ("10 2 /", 5.0),
        ("2 3 + 4 5 + *", 45.0), // (2+3)*(4+5)
    ];

    for (expr, esperado) in expressoes {
        match avaliar_rpn(expr) {
            Ok(resultado) => println!("\"{}\" = {} (esperado: {})", expr, resultado, esperado),
            Err(e) => println!("Erro em \"{}\": {}", expr, e),
        }
    }
}
