# Mutacoes (cx06_13), com as 8 ligas e ATE 2004-11-01. A nova edicao fica sem mutacao propria (ver rotinas.py).
# - a parada nao marca +0xb5: o comparador lado a lado acusa (e a liga das eliminatorias segue no dia a dia da virada);
# - o registro da fase final aponta a Copa America: o comparador lado a lado acusa.
MUTACOES = [
    ('cx06_13_sem_parada',
     '    League(this).set_stopped(1);\n    1\n}',
     '    League(this).set_stopped(0); // mutation\n    1\n}',
     'cx06_13 qualifiers season stop'),
    ('cx06_13_registro_errado',
     '    covered!("cx06_13 qualifiers finals record");\n    comp_record_of_key(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP)',
     '    covered!("cx06_13 qualifiers finals record");\n    comp_record_of_key(KEY_CLUB_COMP_COPA_AMERICA) // mutation',
     'cx06_13 qualifiers finals record'),
]
