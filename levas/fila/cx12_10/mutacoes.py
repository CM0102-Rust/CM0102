# Mutacoes (cx12_10), com a Irlanda (do Norte) ate 2002-08-01:
# - a vaga da repescagem 9 vira 7 e 11 (no lugar de 8 e 10): a carreira quase nao le a previsao ate o fim da temporada;
#   acusada pelo comparador, lado a lado nos 3 bytes da entrada, a cada atualizacao de status.
MUTACOES = [
    ('cx12_10_faixa_da_repescagem',
     'premier_play_off_progress(this, club, current, best, worst, 9, 8, 0xa) // cx12_10: the play-off place',
     'premier_play_off_progress(this, club, current, best, worst, 9, 7, 0xb) // mutation: another band',
     'cx12_10 northern irish premier progress'),
]
