# Mutacoes (cx02_9), carreira com a Irlanda e a Irlanda do Norte ate 2002-08-01. Nenhuma abre caixa.
# - Dundalk fora de Leinster: os inscritos da Copa do Presidente / das faixas da Copa da Liga irlandesa mudam (quem
#   entra em cada grupo) e o comparador acusa;
# - o premio da Copa da Irlanda 200.000: o comparador acusa na virada;
# - a Copa da Irlanda do Norte com premio 50.000: o comparador acusa na virada.
MUTACOES = [
    ('cx02_9_dundalk_fora',
     '    let towns: [&[u8]; 6] = [b"Bray", b"Longford", b"Dundalk", b"Drogheda", b"Athlone", b"Kilkenny"];',
     '    let towns: [&[u8]; 6] = [b"Bray", b"Longford", b"Dundalk_", b"Drogheda", b"Athlone", b"Kilkenny"]; // mutation'),
    ('cx02_9_premio_copa_irlanda',
     '    set_up_prize_rounds(Cup(this), 100_000)',
     '    set_up_prize_rounds(Cup(this), 200_000) // mutation'),
    ('cx02_9_premio_irlanda_do_norte',
     '    set_up_prize_rounds(Cup(this), 0)',
     '    set_up_prize_rounds(Cup(this), 50_000) // mutation'),
]
