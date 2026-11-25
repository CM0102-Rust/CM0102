# Mutacoes (rx12_6), pela carreira com as 8 ligas ate 2002-08-01 (os meses de outubro de 2001 a maio de 2002).
# - o tecnico do mes pela liga inglesa (outro tecnico ganha o premio australiano);
# - o jovem do mes sem limite de idade.
MUTACOES = [
    ('rx12_6_tecnico_ingles',
     '    award_month_manager(nation_award(this, 1), KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE); // rx12_6: the manager of the month',
     '    award_month_manager(nation_award(this, 1), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation',
     'rx12_6 australian month awards'),
    ('rx12_6_jovem_sem_idade',
     '    award_month_player(this, nation_award(this, 3), 0x15); // rx12_6: the young player, up to 21',
     '    award_month_player(this, nation_award(this, 3), -1); // mutation',
     'rx12_6 australian month awards'),
]
