# Mutacoes da leva117 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva117.py --jogo 2001-08-20
MUTACOES = [
    ('e_ocupante', "                        if ocupante == 0 || rating(ocupante, jg(k), i) < r {", "                        if ocupante == 0 || rating(ocupante, jg(k), i) <= r {"),
    ('e_impar', "            let exit = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 10 } else { f == 20 };", "            let exit = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 9 } else { f == 20 };"),
    ('e_melhor', "                if melhor == 0 || r > best_rating {\n                    melhor = cand;", "                if melhor == 0 || r >= best_rating {\n                    melhor = cand;"),
    ('e_perda', "                        if swap == 0 || r_current - outra_nota < smaller_loss {", "                        if swap == 0 || r_current - outra_nota <= smaller_loss {"),
    ('e_extras', "marks, bytes, a14, curtos + 2 * m as usize,\n        );", "marks, bytes, a14, curtos,\n        );"),
    ('e_troca', "                            b_j.wrapping_add(a_i) < a_j.wrapping_add(b_i)", "                            b_j.wrapping_add(a_i) <= a_j.wrapping_add(b_i)"),
    ('e_voltas', "        if !mudou || voltas >= 100 {", "        if !mudou || voltas >= 1 {"),
    ('e_tecnico', "        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, staff::CLASSIFICATION) != 0 {", "        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, staff::CLASSIFICATION) == 0 {"),
]
