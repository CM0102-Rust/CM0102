# Mutacoes (sx04c_15), pela carreira com 'ENG SCO WAL' ate 2002-06-01 (os premios do mes de agosto de 2001 a maio de
# 2002).
# - o jovem do mes sem o limite de idade (o mesmo jogador do mes ganha os dois premios);
# - o tecnico do mes de cada divisao pela liga inglesa.
MUTACOES = [
    ('sx04c_15_jovem_sem_idade',
     '        award_month_best_player(this, nation_award(this, scotland_block(d) + 3), *key, 0x15);',
     '        award_month_best_player(this, nation_award(this, scotland_block(d) + 3), *key, -1); // mutation',
     'sx04c_15 scottish month awards'),
    ('sx04c_15_tecnico_ingles',
     '        award_month_manager(nation_award(this, scotland_block(d) + 1), *key);',
     '        award_month_manager(nation_award(this, scotland_block(d) + 1), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); let _ = key; // mutation',
     'sx04c_15 scottish month awards'),
]
