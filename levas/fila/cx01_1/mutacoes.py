# Mutacoes (cx01_1), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - a importancia padrao 21 e nao 20: o comparador do construtor acusa na criacao do jogo;
# - a copa nasce com a importancia 19: o comparador do Cup::Cup acusa na criacao do jogo;
# - reservas e substituicoes trocadas de nibble na partida: o banco das partidas muda (a carreira e o comparador).
MUTACOES = [
    ('cx01_1_importancia_21',
     '    comp.set_importance(0x14);\n    comp.set_prize_fund(0);',
     '    comp.set_importance(0x15); // mutation\n    comp.set_prize_fund(0);'),
    ('cx01_1_copa_importancia_19',
     '    new_comp(this);\n    let cup = Cup(this);',
     '    new_comp(this);\n    League(this).set_importance(0x13); // mutation\n    let cup = Cup(this);'),
    ('cx01_1_nibbles_trocados',
     '    fixture.set_squad_rules((fixture.squad_rules() & 0xff00) | allowed << 4 | named);',
     '    fixture.set_squad_rules((fixture.squad_rules() & 0xff00) | named << 4 | allowed); // mutation'),
]
