# Mutacoes (cqu_7): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte. Na Copa da Franca, um
# ano adiante derrubava o jogo (excecao em 0x9480d0 na criacao): trocada pelo dobro do fundo de premios (outros premios pagos).
MUTACOES = [
    ('cqu7_copa_da_franca_outro_premio',
     'cqu7_p32(this + 0x28, 0xf4240);',
     'cqu7_p32(this + 0x28, 0x1e8480); // mutacao: o dobro do fundo de premios',
     'cqu_7 copa_da_franca nova'),
    ('cqu7_copa_da_liga_francesa_um_ano_adiante',
     'cqu7_copa_da_liga_francesa_corpo(this, ano, dono) // cqu7: copa_da_liga_francesa',
     'cqu7_copa_da_liga_francesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_7 copa_da_liga_francesa nova'),
    ('cqu7_supercopa_francesa_um_ano_adiante',
     'cqu7_supercopa_francesa_corpo(this, ano, dono) // cqu7: supercopa_francesa',
     'cqu7_supercopa_francesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_7 supercopa_francesa nova'),
]
