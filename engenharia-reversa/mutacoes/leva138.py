# Mutacoes da leva138 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva138.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('b_celula', "0x5e + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;", "0x5e + (t as i32 + l as i32 * 2) as usize * 4) as *mut i32;", "batedor do clube"),
    ('b_limite', "if v > (*(G_STAFF_COUNT as *const i32)).wrapping_add(0x52d) {", "if v > (*(G_STAFF_COUNT as *const i32)).wrapping_add(0x52c) {", "batedor alem das pessoas"),
    ('b_ficha', "    if *((person + staff::PLAYER) as *const u32) == 0 {\n        crate::comparison::branch(\"batedor sem ficha\");\n        *cell = -1;", "    if *((person + staff::PLAYER) as *const u32) == 0 {\n        crate::comparison::branch(\"batedor sem ficha\");\n        *cell = -2;", "batedor sem ficha"),
    ('b_clube', "    if *((person + staff::CLUB_JOB) as *const usize) == clube {\n        crate::comparison::branch(\"batedor do clube\");", "    if *((person + staff::CLUB_JOB) as *const usize) != clube {\n        crate::comparison::branch(\"batedor do clube\");", "batedor do clube"),
    ('b_saiu', "    crate::comparison::branch(\"batedor saiu do clube\");\n    *cell = -1;\n    0", "    crate::comparison::branch(\"batedor saiu do clube\");\n    0", "batedor saiu do clube"),
    ('i_ocupados', "                if *u(s, 0x38) == 0 {\n                    *ocupados = (*ocupados).wrapping_add(1);", "                if *u(s, 0x38) == 0 {\n                    *ocupados = (*ocupados).wrapping_add(2);", "instructions guardadas"),
    ('i_menos_um', "        for i in 0..20 {\n            *instrucao(i) = u32::MAX;", "        for i in 0..19 {\n            *instrucao(i) = u32::MAX;"),
    ('i_devolve', "                for i in 0..20 {\n                    *instrucao(i) = *u(s, 0x3c + 4 * i);", "                for i in 1..20 {\n                    *instrucao(i) = *u(s, 0x3c + 4 * i);", "instructions devolvidas"),
    ('i_libera', "                crate::comparison::branch(\"instructions liberadas\");\n                *u(s, 0x38) = 0;", "                crate::comparison::branch(\"instructions liberadas\");", "instructions liberadas"),
    ('n_zero', "        0 => 4 + s,", "        0 => 5 - s,", "numero da competicao"),
    ('n_1ef', "        0x1ef => s,", "        0x1ef => 1 - s,", "numero da competicao"),
    ('n_3de', "        0x3de => 2 + s,", "        0x3de => 3 + s,", "numero da competicao"),
]
