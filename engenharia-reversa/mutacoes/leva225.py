# Mutações da 225a leva (itens db_q10, db_q11, db_q12). Uso: python mutacoes.py mutacoes\leva225.py --carreira 2002-06-01
MUTACOES = [
    ('dq10_contar', '            n = n.wrapping_add(1);\n        }\n        n\n', '            n = n.wrapping_add(2);\n        }\n        n\n'),
    ('dq10_reunir', 'array.add(i), squad_slot(Club(club), i)', 'array.add(i), squad_slot(Club(club), 49 - i)'),
    ('dq10_dirigido', '    (*((read_usize(club + 0x53) + 0x11c) as *const u8) & 2 != 0) as u8', '    (*((read_usize(club + 0x53) + 0x11c) as *const u8) & 4 != 0) as u8'),
    ('dq11_nao_jogador', '        last = read_u8(array + 3 * k);', '        last = read_u8(array + 3 * k + 1);'),
    ('dq11_jogador', 'write_u8(player + d as usize, read_u8(array + 3 * i));', 'write_u8(player + d as usize, read_u8(array + 3 * i + 1));'),
    ('dq11_normalizar', '    let delta = (-5i32).wrapping_sub', '    let delta = (-4i32).wrapping_sub'),
    ('dq12_familia', '    match colour_family(colour) {\n        Some(f) => f,', '    match colour_family(colour) {\n        Some(f) => f % 10 + 1,'),
    ('dq12_confundem', '    if fa == fb || par(8, 9)', '    if fa != fb || par(8, 9)'),
    ('dq12_atributo', '(b + b)) * 0.05 + 10.0;', '(b + b)) * 0.05 + 11.0;'),
]
