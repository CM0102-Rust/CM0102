# Mutacoes da leva117 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva117.py --jogo 2001-08-20
MUTACOES = [
    ('e_ocupante', "                        if ocupante == 0 || nota(ocupante, jg(k), i) < r {", "                        if ocupante == 0 || nota(ocupante, jg(k), i) <= r {"),
    ('e_impar', "            let sair = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 10 } else { f == 20 };", "            let sair = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 9 } else { f == 20 };"),
    ('e_melhor', "                if melhor == 0 || r > melhor_nota {\n                    melhor = cand;", "                if melhor == 0 || r >= melhor_nota {\n                    melhor = cand;"),
    ('e_perda', "                        if troca == 0 || r_atual - outra_nota < menor_perda {", "                        if troca == 0 || r_atual - outra_nota <= menor_perda {"),
    ('e_extras', "marcas, bytes, a14, curtos + 2 * m as usize,\n        );", "marcas, bytes, a14, curtos,\n        );"),
    ('e_troca', "                            b_j.wrapping_add(a_i) < a_j.wrapping_add(b_i)", "                            b_j.wrapping_add(a_i) <= a_j.wrapping_add(b_i)"),
    ('e_voltas', "        if !mudou || voltas >= 100 {", "        if !mudou || voltas >= 1 {"),
    ('e_tecnico', "        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {", "        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) == 0 {"),
]
