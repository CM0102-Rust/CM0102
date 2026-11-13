# Mutacoes (mwc_6).
MUTACOES = [
    ('mwc6_disciplina_inglesa',
     'discipline: discipline_rule::DENMARK, // mwc6: the Danish First Division discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc6_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc6: the Danish First Division points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_6 den first rules'),
    ('mwc6_cota_dez_vezes',
     '    eng_prm_fill_league_table(League(this), 0x10, 0x9b8e30, 0x142, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 0x2710, || {',
     '    eng_prm_fill_league_table(League(this), 0x10, 0x9b8e30, 0x142, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 0x186a0, || { // mutation: ten times the prize share',
     'cqi_5 full table'),
]
