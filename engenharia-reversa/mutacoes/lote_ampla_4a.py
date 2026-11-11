# Lote 4a da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4a.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('c_k0_b', '                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }', '                if v > 0x4b { 1 } else if v >= 0x28 { 3 } else if v >= 10 { 3 } else { low(-30) }'),
    ('c_tipo5_b', '    if c.contract_type() & 0x3f == 5 {\n        return 8;', '    if c.contract_type() & 0x3f == 5 {\n        return 7;'),
    ('o_campos', '    for o in [0x36, 0x38, 0x34] {', '    for o in [0x36, 0x34, 0x38] {'),
    ('r_banco', '            cod2[i] = w(curtos, j);', '            cod2[i] = w(codes, j);'),
    ('r_idade', '            3 => !p.is_null() && read_i8(rule + 1) < (p.age() as i8),', '            3 => !p.is_null() && read_i8(rule + 1) <= (p.age() as i8),'),
    ('n_veterano', '            Some(0x9689e0)', '            Some(0x968968)'),
    ('t_item', '        if *p == read_i32(e) {', '        if *p == read_i32(e + 4) {'),
    ('m_marca', '    (t.promoted_count() != 0 && (t.league_flags() as u8) & 2 != 0) as u32', '    (t.promoted_count() != 0 && (t.league_flags() as u8) & 1 != 0) as u32'),
    ('f_estado', '        2 | 3 | 4 | 0x10 | 0x14 => 1,', '        2 | 3 | 0x10 | 0x14 => 1,'),
]
