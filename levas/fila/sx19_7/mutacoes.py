# Mutacoes (sx19_7), pela carreira ENG SWE FIN NOR POR ate 2002-08-01 (Svenska Cupen):
# - a escala da copa sueca 12 -> 10 (acusada pelo comparador do construtor; na carreira, muda a duracao das suspensoes
#   por pontos dos vermelhos da copa);
# - a copa sueca com a regra da liga (o vermelho direto tambem suspende na hora).
MUTACOES = [
    ('sx19_7_escala',
     'rule_set: 0x1d, scale: 0xc,',
     'rule_set: 0x1d, scale: 0xa, // mutation\n   ',
     'sx19_7 swedish cup body built'),
    ('sx19_7_regra_da_liga',
     'sweden_card_verdict(this, row, fixture, entry, &SWEDEN_CUP_RULING_BODY, true) // sx19_7: the cup\'s red card rule',
     'sweden_card_verdict(this, row, fixture, entry, &SWEDEN_CUP_RULING_BODY, false) // mutation: the league\'s rule',
     'sx19_7 swedish cup cards'),
]
