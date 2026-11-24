# Mutacoes (cx11_4), com a Italia e a base NORMAL ate 2002-09-30:
# - a divisao de cima da C2/B vira a C1/B: a carreira nao le o vinculo de cima (so o sinal, no all_stages_complete, e
#   continua >= 0), entao escapa dela; e acusada pelo comparador das regras, que compara a seco os campos gravados.
# Trocar a de baixo nao serve: a lista das divisoes vira um laco (o all_stages_complete nao para) ou aponta para um
# objeto que nao existe.
# A tabela fica sem mutacao propria, como no cx11_3 (o molde provado no cqj_4; sem fundo nem TV).
MUTACOES = [
    ('cx11_4_c2b_abaixo_da_c1b',
     'let rules = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C); // cx11_4: the C2/B links',
     'let rules = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C); // mutation: C1/B above',
     'cx11_4 serie c2 b rules'),
]
