# Mutacoes (mwc_8).
MUTACOES = [
    ('mwc8_disciplina_inglesa',
     'discipline: discipline_rule::GERMANY_LEAGUE, // mwc8: the 2. Bundesliga discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc8_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc8: the 2. Bundesliga points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_8 ger second rules'),
    ('mwc8_tv_dez_vezes',
     'eng_prm_pay_tv_rights(this, 0x13d620, 0); // mwc8: the TV money of each club',
     'eng_prm_pay_tv_rights(this, 0xc65d40, 0); // mutation: ten times the TV money',
     'cqi_5 full table'),
]
