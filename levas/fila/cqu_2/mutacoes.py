# Mutacoes (cqu_2). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqu_2 <liga> nova"):
# - a Serie C um ano adiante (os grupos e os jogos da Serie C de 2001 saem para 2002);
# - a J2 com outra regra de disciplina (0 no lugar de 0x13: outras suspensoes nos jogos da J2);
# - a MLS com 2 pontos por vitoria no lugar de 3 (outra tabela da MLS de 2001: outros classificados ao mata-mata).
MUTACOES = [
    ('cqu2_serie_c_um_ano_adiante',
     'bra_nat_third_construct_league(League(this), year, record, &BRA_NAT_THIRD_LEAGUE,',
     'bra_nat_third_construct_league(League(this), year + 1, record, &BRA_NAT_THIRD_LEAGUE, /* mutation: one year ahead */',
     'cqu_2 bra_nat_third new'),
    ('cqu2_j2_outra_disciplina',
     '&JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, "cqu_2 jap_j2 new")',
     '&JAP_J2_LEAGUE, 0, "cqu_2 jap_j2 new") /* mutation: another discipline rule */',
     'cqu_2 jap_j2 new'),
    ('cqu2_mls_dois_pontos',
     '(league::WIN_POINTS, 1, 3), // cqu2: the MLS points for a win',
     '(league::WIN_POINTS, 1, 2), // mutation: two points for a win',
     'cqu_2 usa_mls new'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 12 itens cqu/cqv/cqw, impressao f29c85195c2a534f): as tres ACUSADAS (2001-215, 2001-084, 2001-120).
