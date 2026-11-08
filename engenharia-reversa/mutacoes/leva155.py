# Mutacoes da leva155 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva155.py --carreira 2002-06-01
MUTACOES = [
    ('e_limite', "                        if *b > 0xb {\n                            *b = 0xb;", "                        if *b > 0xa {\n                            *b = 0xa;"),
    ('e_opcao', "                    let v = exe::at_94739c(BUF) as u8;", "                    let v = (exe::at_94739c(BUF) as u8).wrapping_add((o == 1) as u8);"),
    ('e_peso', "            let total = (*weight(e)).wrapping_mul(10);", "            let total = (*weight(e)).wrapping_mul(9);"),
    ('e_soma', "                sum += *s as i32;", "                sum += *s as i32 + 1;"),
    ('e_maior', "                let p = (*(s as *const i16) as i32).max(*weight(e));", "                let p = (*(s as *const i16) as i32).min(*weight(e));"),
    ('e_tipo', "                *((s + 2) as *mut i16) = exe::at_94739c(BUF) as i16;", "                *((s + 2) as *mut i16) = exe::at_94739c(BUF) as i16 + 1;"),
]
