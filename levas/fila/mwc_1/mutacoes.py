# Mutacoes (mwc_1).
MUTACOES = [
    ('mwc1_disciplina_inglesa',
     'discipline: discipline_rule::BELGIUM_LEAGUE, // mwc1: the Belgian First Division discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc1_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc1: the Belgian First Division points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_1 bel first rules'),
    ('mwc1_tv_dez_vezes',
     'eng_prm_pay_tv_rights(this, 0xc350, 0); // mwc1: the TV money of each club',
     'eng_prm_pay_tv_rights(this, 0x7a120, 0); // mutation: ten times the TV money',
     'cqi_5 full table'),
]
