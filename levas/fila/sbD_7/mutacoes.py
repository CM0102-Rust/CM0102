# Mutacoes (sbD_7), pela carreira com a Belgica ate 2002-08-01 e pelos comparadores (o plano dos registros contra a
# original; o texto lado a lado):
# - o grupo da repescagem: a linha do clube promovido com o status 1 em vez de 0;
# - a repescagem de rebaixamento: o rebaixado com o status 2 em vez de 3;
# - o texto: "escapa do rebaixamento" e "rebaixado" trocados.
MUTACOES = [
    ('sbD_7_status_do_promovido', '0 => plan(BelSecondRows::Every(0),', '0 => plan(BelSecondRows::Every(1), /* mutation */',
     'sbD_7 belgian second division promotion record'),
    ('sbD_7_status_do_rebaixado', '2 | 0xff => plan(BelSecondRows::First(3),', '2 | 0xff => plan(BelSecondRows::First(2), /* mutation */',
     'sbD_7 belgian second division relegation record'),
    ('sbD_7_textos_trocados', '                1 => club_fate_line(club_id, 0x987594, text),',
     '                1 => club_fate_line(club_id, 0x987564, text), // mutation', 'sbD_7 belgian second division relegation play-off text'),
]
