# Mutacoes (cqu_7): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte. Na Copa da Franca, um
# ano adiante derrubava o jogo (excecao em 0x9480d0 na criacao): trocada pelo dobro do fundo de premios (outros premios pagos).
MUTACOES = [
    ('cqu7_copa_da_franca_outro_premio',
     'show_error_at(0x9ccb28, 0x4c);\n        return this.addr();\n    }\n    this.set_prize_fund(0xf4240);',
     'show_error_at(0x9ccb28, 0x4c);\n        return this.addr();\n    }\n    this.set_prize_fund(0x1e8480); // mutation: double the prize fund',
     'cqu_7 coupe de france new'),
    ('cqu7_copa_da_liga_francesa_um_ano_adiante',
     'construct_fra_lge_cup_body(Cup(this), year, record) // cqu7: the French League Cup',
     'construct_fra_lge_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_7 french league cup new'),
    ('cqu7_supercopa_francesa_um_ano_adiante',
     'construct_fra_super_body(Cup(this), year, record) // cqu7: the French Super Cup',
     'construct_fra_super_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_7 french super cup new'),
]
