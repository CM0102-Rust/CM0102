# Mutações da 225a leva (itens db_q10, db_q11, db_q12). Uso: python mutacoes.py mutacoes\leva225.py --carreira 2002-06-01
MUTACOES = [
    ('dq10_contar', '            n = n.wrapping_add(1);\n        }\n        n\n', '            n = n.wrapping_add(2);\n        }\n        n\n'),
    ('dq10_reunir', 'vetor.add(i), squad_slot(clube, i)', 'vetor.add(i), squad_slot(clube, 49 - i)'),
    ('dq10_dirigido', '    (*((u(0x53) + 0x11c) as *const u8) & 2 != 0) as u8', '    (*((u(0x53) + 0x11c) as *const u8) & 4 != 0) as u8'),
    ('dq11_nao_jogador', '        ultimo = *((vetor + 3 * k) as *const u8);', '        ultimo = *((vetor + 3 * k + 1) as *const u8);'),
    ('dq11_jogador', '*((jogador + d as usize) as *mut u8) = *((vetor + 3 * i) as *const u8);', '*((jogador + d as usize) as *mut u8) = *((vetor + 3 * i + 1) as *const u8);'),
    ('dq11_normalizar', '    let delta = (-5i32).wrapping_sub', '    let delta = (-4i32).wrapping_sub'),
    ('dq12_familia', '        Some(f) => f,', '        Some(f) => f % 10 + 1,'),
    ('dq12_confundem', '    if fa == fb || par(8, 9)', '    if fa != fb || par(8, 9)'),
    ('dq12_atributo', '(b + b)) * 0.05 + 10.0;', '(b + b)) * 0.05 + 11.0;'),
]
