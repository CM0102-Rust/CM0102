# Mutacoes (mwc_4).
MUTACOES = [
    ('mwc4_disciplina_inglesa',
     'discipline: discipline_rule::BELGIUM_LEAGUE, // mwc4: the Belgian Second Division discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc4_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc4: the Belgian Second Division points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_4 bel second rules'),
    ('mwc4_tv_dez_vezes',
     'eng_prm_pay_tv_rights(this.addr(), 0x9c40, 0); // mwc4: the TV money of each club',
     'eng_prm_pay_tv_rights(this.addr(), 0x61a80, 0); // mutation: ten times the TV money',
     'mwc_4 full table'),
]
