# Mutacoes da leva122 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva122.py --jogo 2001-08-20
MUTACOES = [
    ('a_codigo', "                    chamar(FN_MATCH_EVENTS_ADD_EVENT, u(m, 0x8f7a), [0x1f56, m as u32, a, lado_de(j) as u32, u32::MAX]);", "                    chamar(FN_MATCH_EVENTS_ADD_EVENT, u(m, 0x8f7a), [0x1f57, m as u32, a, lado_de(j) as u32, u32::MAX]);"),
    ('a_tirar', "                    tirar(m, j, if byte(m, 0x9049 + 0xd0 * lu) != 0 { -1 } else { -2 });", "                    tirar(m, j, if byte(m, 0x9049 + 0xd0 * lu) != 0 { -2 } else { -1 });"),
    ('a_posicao', "                    *((j + 0x19) as *mut i8) = -2;", "                    *((j + 0x19) as *mut i8) = -3;"),
    ('a_sorteio', "            } else if sortear(4) == 0 || u(j, 0x55) != 0 {", "            } else if sortear(4) == 1 || u(j, 0x55) != 0 {"),
    ('a_par', "                chamar(FN_MATCH_EVENTS_ADD_EVENT, u(m, 0x8f7a), [0x20f6, m as u32, a3, a4, a5 | a6 << 8]);", "                chamar(FN_MATCH_EVENTS_ADD_EVENT, u(m, 0x8f7a), [0x20f6, m as u32, a5, a4, a3 | a6 << 8]);"),
    ('a_minuto', "        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16);\n        forca(m, lado);", "        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16) + 1;\n        forca(m, lado);"),
    ('a_capitao', "            if byte(jv, 0x19) < 0 {", "            if byte(jv, 0x19) <= 0 {"),
]
