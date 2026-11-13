# Mutacoes (mwa_8), nos construtores, carreira ENG IRL NIR ate 2002-08-01: uma por rotina, o ano da copa (um ano
# depois: a edicao de 2001 sai da janela; a Copa do Presidente e a Supercopa da Irlanda do Norte sao jogadas em agosto
# de 2001).
MUTACOES = [
    ('mwa8_presidente_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, "mwa_8 presidents cup new")',
     'construct_irish_cup(Cup(this), year + 1, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, "mwa_8 presidents cup new")',
     'mwa_8 presidents cup new'),
    ('mwa8_supercopa_irlanda_do_norte_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, "mwa_8 northern irish charity shield new")',
     'construct_irish_cup(Cup(this), year + 1, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, "mwa_8 northern irish charity shield new")',
     'mwa_8 northern irish charity shield new'),
    ('mwa8_copa_irlanda_do_norte_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, "mwa_8 irish cup new")',
     'construct_irish_cup(Cup(this), year + 1, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, "mwa_8 irish cup new")',
     'mwa_8 irish cup new'),
]
