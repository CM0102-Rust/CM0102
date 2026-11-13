# Mutacoes (mwc_10).
MUTACOES = [
    ('mwc10_disciplina_inglesa',
     'discipline: discipline_rule::HOLLAND, // mwc10: the Dutch Eerste Divisie discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc10_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc10: the Dutch Eerste Divisie points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_10 hol first rules'),
    ('mwc10_linhas_0_e_1_trocadas',
     '    (fra_first_fill_league_table(this, 0x9cee08, 0x19d, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) == Some(true)) as u32',
     '    let full = fra_first_fill_league_table(this, 0x9cee08, 0x19d, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) == Some(true);\n    if full { let t = this.table(); for k in 0..0x3busize { let (x, y) = (read_u8(t + k), read_u8(t + 0x3b + k)); write_u8(t + k, y); write_u8(t + 0x3b + k, x); } } // mutation: rows 0 and 1 swapped\n    full as u32',
     'mwc_1 new division'),
]
