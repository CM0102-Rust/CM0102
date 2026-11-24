# Mutacoes (cx05_17), carreira ENG ate 2002-06-01.
# - a data da vaga construida 4 bytes adiante: as 127 vagas ficam com a data em +8 (o comparador lado a lado da vaga
#   acusa na criacao do jogo).
MUTACOES = [
    ('cx05_17_data_da_vaga_adiante',
     "    exe::date_date_54a110(this + tournament_slot::START_DATE); // cx05_17: the slot's start date",
     "    exe::date_date_54a110(this + tournament_slot::START_DATE + 4); // mutation"),
]
