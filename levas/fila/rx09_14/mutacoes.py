# Mutacao (rx09_14), com as 8 ligas ate 2002-06-01: a reputacao nacional do jogador do ano da Primeira 9000 em vez de
# 6000 (o vencedor de 16 de dezembro de 2001 ganha mais reputacao; a carreira acusa pelo banco de dados). Os premios
# estaduais saem em 10 de julho de 2001, sem estatisticas: uma mutacao neles nao seria acusada.
MUTACOES = [
    ('rx09_14_jogador_do_ano_9000',
     'year(1, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_PLAYER_OF_THE_YEA, 6000, 3500, first, 0),',
     'year(1, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_PLAYER_OF_THE_YEA, 9000, 3500, first, 0), // mutation: 9000',
     'rx09_14 brazilian awards built'),
]
