# Lote 4a da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4a.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('c_k0_b', '                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }', '                if v > 0x4b { 1 } else if v >= 0x28 { 3 } else if v >= 10 { 3 } else { low(-30) }'),
    ('c_tipo5_b', '    if *((c + 0x35) as *const u8) & 0x3f == 5 {\n        return 8;', '    if *((c + 0x35) as *const u8) & 0x3f == 5 {\n        return 7;'),
    ('o_campos', '    for o in [0x36, 0x38, 0x34] {', '    for o in [0x36, 0x34, 0x38] {'),
    ('r_banco', '            cod2[i] = w(curtos, j);', '            cod2[i] = w(codigos, j);'),
    ('r_idade', '            3 => p != 0 && r(1) < *((p + 0x18) as *const i8),', '            3 => p != 0 && r(1) <= *((p + 0x18) as *const i8),'),
    ('n_veterano', '            Some(0x9689e0)', '            Some(0x968968)'),
    ('t_item', '        if *p == *(e as *const i32) {', '        if *p == *((e + 4) as *const i32) {'),
    ('m_marca', '    (*((t + 0xbe) as *const u8) != 0 && *((t + 0xd9) as *const u8) & 2 != 0) as u32', '    (*((t + 0xbe) as *const u8) != 0 && *((t + 0xd9) as *const u8) & 1 != 0) as u32'),
    ('f_estado', '        2 | 3 | 4 | 0x10 | 0x14 => 1,', '        2 | 3 | 0x10 | 0x14 => 1,'),
]
