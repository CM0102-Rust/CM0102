# Mutacoes da leva138 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva138.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('b_celula', "tactics_row::SET_PIECE_TAKERS + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;", "tactics_row::SET_PIECE_TAKERS + (t as i32 + l as i32 * 2) as usize * 4) as *mut i32;", "taker of the club"),
    ('b_limite', "if v > read_i32(G_STAFF_COUNT).wrapping_add(0x52d) {", "if v > read_i32(G_STAFF_COUNT).wrapping_add(0x52c) {", "taker beyond the people"),
    ('b_ficha', "    if (person.player().addr() as u32) == 0 {\n        crate::comparison::branch(\"taker without record\");\n        *cell = -1;", "    if (person.player().addr() as u32) == 0 {\n        crate::comparison::branch(\"taker without record\");\n        *cell = -2;", "taker without record"),
    ('b_clube', "    if person.club_job() == Club(club) {\n        crate::comparison::branch(\"taker of the club\");", "    if person.club_job() != Club(club) {\n        crate::comparison::branch(\"taker of the club\");", "taker of the club"),
    ('b_saiu', "    crate::comparison::branch(\"taker left the club\");\n    *cell = -1;\n    0", "    crate::comparison::branch(\"taker left the club\");\n    0", "taker left the club"),
    ('i_ocupados', "                if read_u32(s + 0x38) == 0 {\n                    *ocupados = (*ocupados).wrapping_add(1);", "                if read_u32(s + 0x38) == 0 {\n                    *ocupados = (*ocupados).wrapping_add(2);", "instructions stored"),
    ('i_menos_um', "        for i in 0..20 {\n            *instruction(i) = u32::MAX;", "        for i in 0..19 {\n            *instruction(i) = u32::MAX;"),
    ('i_devolve', "                for i in 0..20 {\n                    *instruction(i) = read_u32(s + 0x3c + 4 * i);", "                for i in 1..20 {\n                    *instruction(i) = read_u32(s + 0x3c + 4 * i);", "instructions returned"),
    ('i_libera', "                crate::comparison::branch(\"instructions released\");\n                write_u32(s + 0x38, 0);", "                crate::comparison::branch(\"instructions released\");", "instructions released"),
    ('n_zero', "        0 => 4 + s,", "        0 => 5 - s,", "competition number"),
    ('n_1ef', "        0x1ef => s,", "        0x1ef => 1 - s,", "competition number"),
    ('n_3de', "        0x3de => 2 + s,", "        0x3de => 3 + s,", "competition number"),
]
