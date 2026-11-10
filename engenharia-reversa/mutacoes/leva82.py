# Mutacoes da leva82 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva82.py
MUTACOES = [
    ('t_11', "    (kind != 11) as u32", "    (kind != 12) as u32"),
    ('t_15', "    matches!(kind, 11 | 12 | 13 | 15) as u32", "    matches!(kind, 11 | 12 | 13 | 14) as u32"),
    ('s_2', "    } else if and_second {\n        2", "    } else if and_second {\n        1"),
    ('s_flag', "    if flag != 0 {\n        1\n    } else if and_second", "    if flag != 0 && !and_second {\n        1\n    } else if and_second"),
    ('m_110', "        .wrapping_add((read_i32(person) as usize).wrapping_mul(110))", "        .wrapping_add((read_i32(person) as usize).wrapping_mul(111))"),
    ('m_20', "        .wrapping_add((k as isize * 20) as usize)", "        .wrapping_add((k as u8 as isize * 20) as usize)"),
    ('r_guarda', "                write_i32(G_EMPTY_COMMON_NAME_INDEX, k);\n", ""),
    ('r_60', "                return base().wrapping_add(k as usize * 60);", "                return base().wrapping_add(k as usize * 61);"),
    ('r_sorteio', "            None => i = random(n),", "            None => i = random(n - 1),"),
]
