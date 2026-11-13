# Mutacoes (mwc_5).
MUTACOES = [
    ('mwc5_disciplina_inglesa',
     'discipline: discipline_rule::DENMARK, // mwc5: the Danish Superliga discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc5_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc5: the Danish Superliga points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_5 den prm rules'),
    ('mwc5_tv_dez_vezes',
     'eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwc5: the TV money of each club',
     'eng_prm_pay_tv_rights(this, 0xf4240, 0); // mutation: ten times the TV money',
     'cqi_5 full table'),
]
