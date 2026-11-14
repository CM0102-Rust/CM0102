# Mutacoes (nma_6). A rotina roda 372 vezes na carreira ENG ate 2002-06-01; a historia 0x1f vem do boato de
# transferencia (process_transfer_interest, com a resposta da IA) e as outras das historias novas da IA (0x72d1ed):
# - os efeitos da resposta 3 as outras historias com as marcas da resposta 1 (0x500 em vez de 0x404);
# - os efeitos da resposta 2 as outras historias com as marcas da resposta 3 (0x404 em vez de 0x480);
# - o humor do elenco depois do boato com outra resposta: sorteio(6) em vez de sorteio(5) (muda quem perde moral);
# - os efeitos da resposta 1 as outras historias com as marcas 0x480 em vez de 0x500.
# As duas primeiras trocaram (294a leva) as mutacoes do ramo "transfer interest kept", que rodou uma vez na base e
# atras de condicoes que ela nao mediu; ver levas/294/escapadas.py e levas/294/trocas_prova.txt.
MUTACOES = [
    ('nma6_marcas_da_resposta_3',
     '        3 => 0x404,',
     '        3 => 0x500, // mutation: answer 3 with the flags of answer 1',
     'nma_6 other story'),
    ('nma6_marcas_da_resposta_2',
     '            0x480\n        }',
     '            0x404 // mutation: answer 2 with the flags of answer 3\n        }',
     'nma_6 other story'),
    ('nma6_humor_do_elenco',
     '5 // nma6: the squad\'s mood after the rumour',
     '6 // mutation: draw(6)',
     'nma_6 transfer interest other answer'),
    ('nma6_outras_marcas',
     '1 => 0x500, // nma6: the flags of answer 1',
     '1 => 0x480, // mutation: the flags of answer 2',
     'nma_6 other story'),
]
