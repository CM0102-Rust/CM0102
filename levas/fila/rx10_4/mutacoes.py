# Mutacoes (rx10_4), com a Alemanha e a base NORMAL ate 2002-08-01. Nenhuma abre caixa.
# - a Regionalliga com as regras disciplinares da copa alema: as suspensoes dos jogadores dela mudam (a carreira);
# - um turno so: os jogos da Regionalliga mudam desde a criacao (a carreira; o comparador das regras acusa);
# - a tabela Norte na ordem inversa: os confrontos de cada rodada mudam (a carreira).
MUTACOES = [
    ('rx10_4_disciplina_da_copa',
     'pub(crate) const GER_REGIONAL: DividedLeague = DividedLeague { vtable: 0x96b250, discipline: discipline_rule::GERMANY_LEAGUE, sections: 1, set_up_rules: 0x5dd9f0,',
     'pub(crate) const GER_REGIONAL: DividedLeague = DividedLeague { vtable: 0x96b250, discipline: discipline_rule::GERMANY_CUP, sections: 1, set_up_rules: 0x5dd9f0, // mutation',
     'rx10_4 new regionalliga'),
    ('rx10_4_um_turno',
     '    let mut v = vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0)];',
     '    let mut v = vec![(0x3c, 2, 1), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0)]; // mutation',
     'rx10_4 regionalliga rules'),
    ('rx10_4_tabela_invertida',
     '            exe::league_init_table_row(this, row as u8 as u32, club, 0, 0);\n            if mark_b_teams {',
     '            exe::league_init_table_row(this, (this.club_count() as i8 - 1 - row) as u8 as u32, club, 0, 0); // mutation\n            if mark_b_teams {',
     'rx10_4 regionalliga north table'),
]
