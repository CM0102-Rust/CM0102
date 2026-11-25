# Mutacoes (rx09_13), com as 8 ligas ate 2002-06-01. Os premios dao reputacao (o molde do sx03_2): a carreira acusa
# pelo banco de dados.
# - o jogador do ano nacional pela estatistica 2 (gols) em vez de 0x11: o artilheiro ganha os dois (16 de dezembro de
#   2001); os estaduais de 10 de julho de 2001 nao servem (o jogo comeca sem estatisticas dos estaduais);
# - o jovem do ano ate 23 em vez de 21 (16 de dezembro de 2001): outro vencedor.
MUTACOES = [
    ('rx09_13_jogador_do_ano_pelos_gols',
     'award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]); // rx09_13: the player of the year',
     'award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 2, -1, -1, -1, -1]); // mutation: by goals',
     'rx09_13 brazilian national awards'),
    ('rx09_13_jovem_ate_23',
     'nation_award(this, first + 3), [0x15, 0, 0x11, -1, -1, -1, -1]); // rx09_13: young player up to 21',
     'nation_award(this, first + 3), [0x17, 0, 0x11, -1, -1, -1, -1]); // mutation: up to 23',
     'rx09_13 brazilian national awards'),
]
