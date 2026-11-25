# Mutacoes (rx10_1), carreira ENG com a base NORMAL ate 2002-06-01. Nenhuma abre caixa.
# - a Conference com as regras disciplinares francesas: as suspensoes dos jogadores dela mudam (a carreira);
# - um turno so: os jogos da Conference mudam desde a criacao (a carreira; o comparador das regras acusa);
# - a TV paga 200.000 e nao 100.000: o caixa dos clubes da Conference muda na criacao (a carreira: o mercado deles).
MUTACOES = [
    ('rx10_1_disciplina_francesa',
     '    l.set_discipline_rule(discipline_rule::ENGLAND);\n    if exe::league_load_from_temp_file(l, 1) != 0 {\n        return this;\n    }\n    covered!("rx10_1 new conference");',
     '    l.set_discipline_rule(discipline_rule::FRANCE); // mutation\n    if exe::league_load_from_temp_file(l, 1) != 0 {\n        return this;\n    }\n    covered!("rx10_1 new conference");',
     'rx10_1 new conference'),
    ('rx10_1_um_turno',
     '    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 3)];',
     '    let mut v = vec![(0x3c, 2, 1), (0xc4, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 3)]; // mutation',
     'rx10_1 conference rules'),
    ('rx10_1_tv_dobrada',
     '|| eng_prm_pay_tv_rights(this, 0x186a0, 0)) // rx10_1: 22 clubs, 5,000 a place, 100,000 TV',
     '|| eng_prm_pay_tv_rights(this, 0x30d40, 0)) // mutation',
     'rx10_1 conference table'),
]
