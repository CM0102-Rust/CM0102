# Mutacoes (cpu_6), todas no vento do clima da partida (0x923010, gravado em +0x4756 do motor na preparacao de cada
# partida). O motor le esse byte nos chutes de longe (vento 2: sorteio(5) e sorteio(2); 3: sorteio(10) e sorteio(5); 4:
# sorteio(15) e sorteio(10); os outros, nada) e em outros lances. Vento fraco virando moderado acrescenta sorteios;
# moderado virando forte e forte virando o mais forte trocam o tamanho dos sorteios. A partida muda e a carreira toda
# muda. Escapa so se nenhuma partida ate 2002-06-01 cair na band (os contadores mostram).
MUTACOES = [
    ('cpu6_vento_fraco_moderado', '            1 // cpu6: o vento fraco',
     '            2 // mutacao: moderado',
     'cpu_6 light wind'),
    ('cpu6_vento_moderado_forte', '            2 // cpu6: o vento moderado',
     '            3 // mutacao: forte',
     'cpu_6 moderate wind'),
    ('cpu6_vento_forte_mais', '            3 // cpu6: o vento forte',
     '            4 // mutacao: o mais forte',
     'cpu_6 strong wind'),
]
