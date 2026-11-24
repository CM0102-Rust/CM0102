# Mutacoes (sx19_9), pela carreira ENG POL TUR GRE HRV RUS ate 2002-08-01:
# - a liga turca suspendendo a cada terceiro amarelo (e nao a cada quarto);
# - a Russia com o ciclo de tres amarelos (a contagem que o comparador confere e a que o aviso de um cartao da
#   suspensao le; na carreira nao muda suspensao nenhuma, so o comparador acusa).
MUTACOES = [
    ('sx19_9_turquia_terceiro',
     'stepped_card_verdict(this, row, fixture, entry, &TURKEY_LEAGUE_RULING_BODY, 4) // sx19_9: every fourth Turkish league booking',
     'stepped_card_verdict(this, row, fixture, entry, &TURKEY_LEAGUE_RULING_BODY, 3) // mutation: every third',
     'sx19_9 turkish league cards'),
    ('sx19_9_russia_ciclo_de_tres',
     'if count >= 4 { // sx19_9: the Russian cycle of four bookings\n        covered!("sx19_9 russian booking cycle");\n        count -= 4;',
     'if count >= 3 { // mutation: a cycle of three\n        covered!("sx19_9 russian booking cycle");\n        count -= 3;',
     'sx19_9 russian booking cycle'),
]
