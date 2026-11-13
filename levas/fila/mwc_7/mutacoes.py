# Mutacoes (mwc_7).
MUTACOES = [
    ('mwc7_disciplina_inglesa',
     'discipline: discipline_rule::DENMARK, // mwc7: the Danish Second Division discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc7_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc7: the Danish Second Division points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_7 den second rules'),
    ('mwc7_cota_dez_vezes',
     '    eng_prm_fill_league_table(League(this), 0x10, 0x9b8ea4, 0x149, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, 0x1388, || {',
     '    eng_prm_fill_league_table(League(this), 0x10, 0x9b8ea4, 0x149, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, 0xc350, || { // mutation: ten times the prize share',
     'cqi_5 full table'),
]
