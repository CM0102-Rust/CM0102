# Mutacoes (sx04c_8), pela carreira com 'ENG SWE FIN NOR POR' ate 2002-06-01 (os premios do mes de abril e maio de
# 2002).
# - o jogador do mes da Premier Division entre os jovens (ate 20);
# - o tecnico do mes da Premier Division pela liga inglesa.
MUTACOES = [
    ('sx04c_8_jovem',
     '    award_month_player(this, nation_award(this, 7), -1);',
     '    award_month_player(this, nation_award(this, 7), 0x14); // mutation',
     'sx04c_8 norwegian month awards'),
    ('sx04c_8_tecnico_ingles',
     '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION);',
     '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation',
     'sx04c_8 norwegian month awards'),
]
