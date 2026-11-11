# Mutacoes (cqu_1). Os quatro construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqu_1 <liga> nova"):
# - a Serie A um ano adiante (os jogos da Serie A de 2001 saem para 2002: outros jogos e outra contagem de sorteios);
# - a Serie B com outra regra de disciplina (0 no lugar de 6: outras suspensoes nos jogos da Serie B);
# - a J1 um ano adiante (os jogos da J1 saem para o ano seguinte);
# - a NSL com outra regra de disciplina (0 no lugar de 0x22).
MUTACOES = [
    ('cqu1_serie_a_um_ano_adiante',
     'bra_nat_first_construct_league(League(this), year, record, &BRA_NAT_FIRST_LEAGUE,',
     'bra_nat_first_construct_league(League(this), year + 1, record, &BRA_NAT_FIRST_LEAGUE, /* mutation: one year ahead */',
     'cqu_1 bra_nat_first new'),
    ('cqu1_serie_b_outra_disciplina',
     '&BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, "cqu_1 bra_nat_second new")',
     '&BRA_NAT_SECOND_LEAGUE, 0, "cqu_1 bra_nat_second new") /* mutation: another discipline rule */',
     'cqu_1 bra_nat_second new'),
    ('cqu1_j1_um_ano_adiante',
     'bra_nat_first_construct_league(League(this), year, record, &JAP_J1_LEAGUE,',
     'bra_nat_first_construct_league(League(this), year + 1, record, &JAP_J1_LEAGUE, /* mutation: one year ahead */',
     'cqu_1 jap_j1 new'),
    ('cqu1_nsl_outra_disciplina',
     '&AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, "cqu_1 aus_nsl new")',
     '&AUS_NSL_LEAGUE, 0, "cqu_1 aus_nsl new") /* mutation: another discipline rule */',
     'cqu_1 aus_nsl new'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 12 itens cqu/cqv/cqw, impressao f29c85195c2a534f): as quatro ACUSADAS (2001-189, 2001-216, 2001-057, 2001-304).
