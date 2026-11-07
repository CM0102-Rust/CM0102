# Mutacoes (cqv_9): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv9_trofeu_da_liga_um_ano_adiante',
     'cqv9_trofeu_da_liga_corpo(this, ano, dono) // cqv9: trofeu_da_liga',
     'cqv9_trofeu_da_liga_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_9 trofeu_da_liga nova'),
    ('cqv9_liga_dos_campeoes_um_ano_adiante',
     'cqv9_liga_dos_campeoes_corpo(this, ano, dono) // cqv9: liga_dos_campeoes',
     'cqv9_liga_dos_campeoes_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_9 liga_dos_campeoes nova'),
]
