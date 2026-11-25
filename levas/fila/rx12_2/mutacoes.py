# Mutacoes (rx12_2), pela carreira com a Finlandia ate 2002-08-01 (os meses de maio a outubro de 2001 que caem na
# janela: agosto a outubro).
# - o tecnico do mes da Premier Division pela liga inglesa (outro tecnico ganha o premio);
# - o jogador do mes da Premier Division so ate 21 anos;
# - sem a equipe da semana da First Division.
MUTACOES = [
    ('rx12_2_tecnico_ingles',
     '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION); // rx12_2: the Premier Division manager',
     '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation',
     'rx12_2 finnish month awards'),
    ('rx12_2_so_jovens',
     '    award_month_player(this, nation_award(this, 5), -1); // rx12_2: the Premier Division player',
     '    award_month_player(this, nation_award(this, 5), 0x15); // mutation',
     'rx12_2 finnish month awards'),
    ('rx12_2_so_premier',
     '    nation_awards_team_of_the_week_of(this, award, &[8, 13]);',
     '    nation_awards_team_of_the_week_of(this, award, &[8]); // mutation'),
]
