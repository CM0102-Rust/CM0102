# Mutacoes (rx10_7), com Portugal e a base NORMAL ate 2002-08-01. Nenhuma abre caixa.
# - a Segunda B com as regras disciplinares francesas: as suspensoes dos jogadores dela mudam (a carreira);
# - um turno so: os jogos da Segunda B mudam desde a criacao (a carreira; o comparador das regras acusa).
MUTACOES = [
    ('rx10_7_disciplina_francesa',
     'pub(crate) const SEGUNDA_B: DividedLeague = DividedLeague { vtable: 0x96e858, discipline: discipline_rule::PORTUGAL, sections: 2, set_up_rules: 0x7d2610,',
     'pub(crate) const SEGUNDA_B: DividedLeague = DividedLeague { vtable: 0x96e858, discipline: discipline_rule::FRANCE, sections: 2, set_up_rules: 0x7d2610, // mutation',
     'rx10_7 new segunda b'),
    ('rx10_7_um_turno',
     '    let mut v = vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0)];',
     '    let mut v = vec![(0x3c, 2, 1), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0)]; // mutation',
     'rx10_7 segunda b rules'),
]
