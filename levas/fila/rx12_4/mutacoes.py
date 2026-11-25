# Mutacoes (rx12_4), pela carreira com a Alemanha ate 2002-08-01 (os meses de agosto de 2001 a maio de 2002).
# - o jogador do mes da primeira divisao so ate 21 anos;
# - o da segunda divisao sem limite de idade.
MUTACOES = [
    ('rx12_4_primeira_so_jovens',
     '    award_month_player(this, nation_award(this, 7), -1); // rx12_4: the First Division player',
     '    award_month_player(this, nation_award(this, 7), 0x15); // mutation',
     'rx12_4 german players of the month'),
    ('rx12_4_segunda_sem_idade',
     '    award_month_player(this, nation_award(this, 11), 0x14); // rx12_4: the Second Division player, up to 20',
     '    award_month_player(this, nation_award(this, 11), -1); // mutation',
     'rx12_4 german players of the month'),
]
