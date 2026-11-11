# Mutacoes (cqv_9): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> new"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv9_trofeu_da_liga_um_ano_adiante',
     'eng_auto_cup_constructor_body(this, year, owner) // cqv9: the League Trophy',
     'eng_auto_cup_constructor_body(this, year + 1, owner) // mutacao: um ano adiante',
     'cqv_9 league trophy new'),
    ('cqv9_liga_dos_campeoes_um_ano_adiante',
     'european_cup_constructor_body(this, year, owner) // cqv9: the Champions League',
     'european_cup_constructor_body(this, year + 1, owner) // mutacao: um ano adiante',
     'cqv_9 champions league new'),
]
