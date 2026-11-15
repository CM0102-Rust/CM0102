# Mutações da 300a leva (itens mwc_14, mwc_15, mwc_16, mwc_17). Uso: python mutacoes.py mutacoes\leva300.py --carreira 2002-06-01
MUTACOES = [
    ('mwc14_belfacup_ano_mais_2', '[0x216, 0x21d, 0x224], 1, true, "mwc_14 bel fa cup turnover") // mwc14: the Belgian Cup', '[0x216, 0x21d, 0x224], 2, true, "mwc_14 bel fa cup turnover") // mutation: the year goes up by 2', 'mwc_14 bel fa cup turnover'),
    ('mwc14_dencup_ano_mais_2', '[0x1fe, 0x205, 0x20c], 1, true, "mwc_14 den cup turnover") // mwc14: the Danish Cup', '[0x1fe, 0x205, 0x20c], 2, true, "mwc_14 den cup turnover") // mutation: the year goes up by 2', 'mwc_14 den cup turnover'),
    ('mwc14_gercup_ano_mais_2', '[0x220, 0x227, 0x22e], 1, true, "mwc_14 ger cup turnover") // mwc14: the German Cup', '[0x220, 0x227, 0x22e], 2, true, "mwc_14 ger cup turnover") // mutation: the year goes up by 2', 'mwc_14 ger cup turnover'),
    ('mwc15_belsuper_ano_mais_2', '[0x134, 0x13b, 0x142], 1, true, "mwc_15 bel super turnover") // mwc15: the Belgian Super Cup', '[0x134, 0x13b, 0x142], 2, true, "mwc_15 bel super turnover") // mutation: the year goes up by 2', 'mwc_15 bel super turnover'),
    ('mwc15_holsuper_ano_mais_2', '[0x121, 0x128, 0x12f], 1, false, "mwc_15 hol super turnover") // mwc15: the Dutch Super Cup', '[0x121, 0x128, 0x12f], 2, false, "mwc_15 hol super turnover") // mutation: the year goes up by 2', 'mwc_15 hol super turnover'),
    ('mwc15_gerlgecup_ano_mais_2', '[0x1ac, 0x1b3, 0x1ba], 1, true, "mwc_15 ger lge cup turnover") // mwc15: the German League Cup', '[0x1ac, 0x1b3, 0x1ba], 2, true, "mwc_15 ger lge cup turnover") // mutation: the year goes up by 2', 'mwc_15 ger lge cup turnover'),
    ('mwc16_belfacup_disciplina_inglesa', '    discipline: discipline_rule::BELGIUM_CUP, // mwc16: the Belgian Cup discipline rule', '    discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule', 'mwc_16 new cup'),
    ('mwc16_dencup_disciplina_inglesa', '    discipline: discipline_rule::DENMARK, // mwc16: the Danish Cup discipline rule', '    discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule', 'mwc_16 new cup'),
    ('mwc16_gercup_disciplina_inglesa', '    discipline: discipline_rule::GERMANY_CUP, // mwc16: the German Cup discipline rule', '    discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule', 'mwc_16 new cup'),
    ('mwc17_belsuper_banco_de_3', '    bench: Some((7, 7)),', '    bench: Some((3, 3)), // mutation: a bench of 3', 'mwc_16 new cup'),
    ('mwc17_holsuper_inscritos_belgas', '    entrants: 0x5f8c10,', '    entrants: 0x4212f0, // mutation: the Belgian Super Cup entrants', 'mwc_16 new cup'),
    ('mwc17_gerlgecup_disciplina_inglesa', '    discipline: discipline_rule::GERMANY_LEAGUE, // mwc17: the German League Cup discipline rule', '    discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule', 'mwc_16 new cup'),
]
