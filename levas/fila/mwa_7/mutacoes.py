# Mutacoes (mwa_7), nos construtores das copas irlandesas, carreira ENG IRL NIR ate 2002-08-01: uma por rotina, o ano
# da copa (um ano depois: a edicao 2001-02 sai da janela e os clubes nao jogam).
MUTACOES = [
    ('mwa7_copa_da_irlanda_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, "mwa_7 irish challenge cup new")',
     'construct_irish_cup(Cup(this), year + 1, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, "mwa_7 irish challenge cup new")',
     'mwa_7 irish challenge cup new'),
    ('mwa7_leinster_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 leinster cup new")',
     'construct_irish_cup(Cup(this), year + 1, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 leinster cup new")',
     'mwa_7 leinster cup new'),
    ('mwa7_munster_ano_seguinte',
     'construct_irish_cup(Cup(this), year, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 munster cup new")',
     'construct_irish_cup(Cup(this), year + 1, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 munster cup new")',
     'mwa_7 munster cup new'),
]
