# Mutacoes (rx13_3), pela carreira com o grupo da Argentina ate 2002-08-01 (os premios nascem na criacao do jogo; a
# equipe da semana sai a cada rodada; a reputacao ganha com um premio muda o mercado):
# - a equipe da semana da segunda divisao escolhida entre os clubes da primeira (a competicao de clubes trocada).
# O construtor fica sem mutacao propria (o molde comum do sx05b_5); o comparador confere a vtable, o id, o tipo e a
# contagem contra a original. O daily_update fica sem mutacao: so repassa ao NationAwards::daily_update, que sem as
# notas do mes (+0xa nulo, o caso da Argentina num jogo novo) nao faz nada; pular a chamada nao muda a carreira.
MUTACOES = [
    ('rx13_3_segunda_pela_primeira',
     '(KEY_STAFF_COMP_ARGENTINIAN_SECOND_DIVISION_TEAM_OF_THE, 1000, KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION),',
     '(KEY_STAFF_COMP_ARGENTINIAN_SECOND_DIVISION_TEAM_OF_THE, 1000, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION), // mutation',
     'rx13_3 argentine awards built'),
]
