# Mutacoes para a sessao do Lucas (TELAS: a cadeia nao roda); os comparadores as acusam na tela dos recordes do clube.
MUTACOES = [
    ('rx08b_2_sem_a_ultima_palavra',
     '    holder(0x371);\n    w(0x389);\n    this\n}',
     '    holder(0x371);\n    this\n}',
     'rx08b_2 season record copy'),
    ('rx08b_2_ano_da_temporada_zero',
     'write_u16(entry(1), exe::key_nation_get_club_season_year(club));',
     'write_u16(entry(1), 0); // rx08b_2 mutation',
     'rx08b_2 history list'),
]
