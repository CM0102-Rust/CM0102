# Mutacoes da leva122 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva122.py --jogo 2001-08-20
MUTACOES = [
    ('a_codigo', "                    call(FN_MATCH_EVENTS_ADD_EVENT, u(m, engine::COMMENTATOR), [0x1f56, m as u32, a, side_of(j) as u32, u32::MAX]);", "                    call(FN_MATCH_EVENTS_ADD_EVENT, u(m, engine::COMMENTATOR), [0x1f57, m as u32, a, side_of(j) as u32, u32::MAX]);"),
    ('a_tirar', "                    remove(m, j, if byte(m, 0x9049 + 0xd0 * lu) != 0 { -1 } else { -2 });", "                    remove(m, j, if byte(m, 0x9049 + 0xd0 * lu) != 0 { -2 } else { -1 });"),
    ('a_posicao', "                    *((j + match_player::POSITION) as *mut i8) = -2;", "                    *((j + match_player::POSITION) as *mut i8) = -3;"),
    ('a_sorteio', "            } else if random(4) == 0 || u(j, match_player::INJURY_CULPRIT) != 0 {", "            } else if random(4) == 1 || u(j, match_player::INJURY_CULPRIT) != 0 {"),
    ('a_par', "                call(FN_MATCH_EVENTS_ADD_EVENT, u(m, engine::COMMENTATOR), [0x20f6, m as u32, a3, a4, a5 | a6 << 8]);", "                call(FN_MATCH_EVENTS_ADD_EVENT, u(m, engine::COMMENTATOR), [0x20f6, m as u32, a5, a4, a3 | a6 << 8]);"),
    ('a_minuto', "        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16);\n        strength(m, lado);", "        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16) + 1;\n        strength(m, lado);"),
    ('a_capitao', "            if byte(jv, match_player::POSITION) < 0 {", "            if byte(jv, match_player::POSITION) <= 0 {"),
]
