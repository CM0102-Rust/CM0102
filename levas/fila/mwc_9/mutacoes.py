# Mutacoes (mwc_9).
MUTACOES = [
    ('mwc9_disciplina_inglesa',
     'discipline: discipline_rule::HOLLAND, // mwc9: the Dutch Eredivisie discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc9_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc9: the Dutch Eredivisie points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_9 hol prm rules'),
    ('mwc9_tv_dez_vezes',
     'eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwc9: the TV money of each club',
     'eng_prm_pay_tv_rights(this, 0x989680, 0); // mutation: ten times the TV money',
     'cqi_5 full table'),
]
