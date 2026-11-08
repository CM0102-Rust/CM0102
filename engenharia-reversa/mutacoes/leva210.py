# Mutações da 210a leva (itens sim_midia_lesao, sim_selecoes_taticas, sim_copas_amistosos). Uso: python mutacoes.py mutacoes\leva210.py --carreira 2002-06-01
MUTACOES = [
    ('m_nenhuma', '    0 // nenhuma história de indisciplina', '    1 // nenhuma história de indisciplina'),
    ('l_empate', '            if r < best {', '            if r <= best {'),
    ('n_5000', '        5000..=5999 => 1,', '        5000..=5999 => 5,'),
    ('c_propria', '            for j in i + 1..50 {', '            for j in i..50 {'),
    ('v_vaga', '        Some(i) => i as u8,', '        Some(i) => i as u8 + 1,'),
    ('m_dobro', '            weight = weight.wrapping_add(weight);', '            weight = weight.wrapping_add(1);'),
    ('r_ou', '    core::ptr::write_unaligned(marks, m);\n    if m & 0x800 == 0 {', '    if m & 0x800 == 0 {'),
    ('c_lados', '        for o in [0, 2] {', '        for o in [2] {'),
    ('p_maior', '                if r > best {', '                if r < best {'),
]
