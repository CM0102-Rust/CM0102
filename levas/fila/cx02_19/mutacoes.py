# Mutacoes (cx02_19), carreira com a Grecia ate 2002-08-01. Nenhuma abre caixa.
# - o banco das outras rodadas com 4 substituicoes e nao 3: o banco das partidas muda (a carreira e o comparador);
# - o 3o lugar de um grupo vira 22 e nao 21: o comparador acusa a cada posicao gravada.
# Os registros de carreira so passam ao molde provado (ItaCup) a fase e a rodada: sem mutacao propria (o comparador
# deles roda so a original, e os historicos nao voltam a simulacao antes da data).
MUTACOES = [
    ('cx02_19_banco_4_substituicoes',
     '    let bench = if fixture.round_name() == 0xa { 0x77 } else { 0x37 };',
     '    let bench = if fixture.round_name() == 0xa { 0x77 } else { 0x47 }; // mutation'),
    ('cx02_19_terceiro_22',
     'GroupCupProgress { knockout: 10, current: &[17, 17, 21, 30],',
     'GroupCupProgress { knockout: 10, current: &[17, 17, 22, 30], // mutation\n   '),
]
