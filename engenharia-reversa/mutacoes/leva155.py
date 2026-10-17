# Mutacoes da leva155 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva155.py --carreira 2002-06-01
MUTACOES = [
    ('e_limite', "                        if *b > 0xb {\n                            *b = 0xb;", "                        if *b > 0xa {\n                            *b = 0xa;"),
    ('e_opcao', "                    let v = numero_curto(BUF) as u8;", "                    let v = (numero_curto(BUF) as u8).wrapping_add((o == 1) as u8);"),
    ('e_peso', "            let total = (*peso(e)).wrapping_mul(10);", "            let total = (*peso(e)).wrapping_mul(9);"),
    ('e_soma', "                soma += *s as i32;", "                soma += *s as i32 + 1;"),
    ('e_maior', "                let p = (*(s as *const i16) as i32).max(*peso(e));", "                let p = (*(s as *const i16) as i32).min(*peso(e));"),
    ('e_tipo', "                *((s + 2) as *mut i16) = numero_curto(BUF) as i16;", "                *((s + 2) as *mut i16) = numero_curto(BUF) as i16 + 1;"),
]
