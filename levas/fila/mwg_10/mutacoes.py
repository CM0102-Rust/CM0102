# Mutacoes (mwg_10) (os comparadores so rodam o original: pegam so pela carreira):
# - a Intertoto nao monta a primeira rodada na virada (as rodadas de junho e julho sem sorteio na hora).
# O fundo da Copa da UEFA (premios a partir de agosto) e o Mundial de Clubes (fim do ano) ficam sem mutacao propria: o
# efeito so aparece depois da data da cadeia; o molde da virada e o de set_up_rounds (ConChamp) ja estao provados.
MUTACOES = [
    ('mwg10_intertoto_sem_primeira_rodada',
     "exe::cup_set_up_round(this, 0) // mwg10: the first round set up",
     "0 // mutation: the first round not set up",
     'mwg_10 intertoto rollover'),
]
