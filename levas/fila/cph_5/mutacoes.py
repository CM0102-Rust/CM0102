# Mutacoes das aposentadorias. A da pendente apaga a ultima com pessoa 0 em vez de -1 (o registro apagado vira uma
# pessoa valida): o comparando acusa na primeira remocao; a carreira, se o vetor for lido alem da contagem. As da
# decisao mudam o limite do tecnico fraco (49) e o do sorteio do jogador (700): devem acusar (aposentadorias mudam o
# banco e os sorteios seguintes).
MUTACOES = [
    ('cph_pendente_fim', 'core::ptr::write_unaligned(fim as *mut u32, u32::MAX); // a última sem pessoa',
     'core::ptr::write_unaligned(fim as *mut u32, 0); // mutacao: pessoa 0'),
    ('cph_desempregado_tecnico', '*((tecnico + 0xa) as *const i16) < 50 && cph5_sortear(5) == 0 // o técnico fraco',
     '*((tecnico + 0xa) as *const i16) < 49 && cph5_sortear(5) == 0 // mutacao: 49'),
    ('cph_desempregado_sorteio', 'cph5_sortear(nota) < 0x2ee // o sorteio pela nota',
     'cph5_sortear(nota) < 700 // mutacao: 700'),
]
