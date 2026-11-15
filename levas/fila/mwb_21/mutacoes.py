# Mutacoes (mwb_21): o mata-mata da Copa Premier na fase 0 em vez da 1 (os dois grupos passam a nao registrar nada nos
# tecnicos). O grupo e mutado pela do mwb_19 (o molde comum). A primeira, mwb21_mata_mata_em_5_rodadas (o peso por
# rodada em 5 em vez de 3), escapou da carreira: ver levas/298/escapadas.py e levas/298/trocas_prova.txt.
MUTACOES = [
    ('mwb21_mata_mata_na_fase_zero', 'welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 1, 3) // mwb21: the Premier Cup knockout over 3 rounds', 'welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 0, 3) // mutation: the knockout at stage 0', 'mwb_21 welsh premier cup staff records'),
]
