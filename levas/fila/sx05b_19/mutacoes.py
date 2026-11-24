# Mutacoes (sx05b_19), pela carreira com Gales ate 2002-06-01.
# - o tecnico do mes pela liga inglesa (outro tecnico ganha o premio galês);
# - sem a equipe da semana galesa.
MUTACOES = [
    ('sx05b_19_tecnico_ingles',
     '    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);',
     '    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation',
     'sx05b_19 welsh manager of the month'),
    ('sx05b_19_sem_semana',
     '    nation_awards_team_of_the_week_of(this, award, &[0]);',
     '    nation_awards_team_of_the_week_of(this, award, &[]); // mutation'),
]
