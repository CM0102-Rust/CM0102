# Mutacoes (cx02_11), carreira com a Irlanda e a Irlanda do Norte ate 2002-08-01. Nenhuma abre caixa.
# - o 2o lugar de um grupo da Copa da Liga irlandesa vira 10 e nao 9: o comparador acusa a cada posicao gravada;
# - a Copa da Liga da Irlanda do Norte trata a fase 3 como grupo: o comparador acusa no mata-mata.
MUTACOES = [
    ('cx02_11_irlanda_segundo_10',
     'GroupCupProgress { knockout: 7, current: &[1, 9, 17, 25],',
     'GroupCupProgress { knockout: 7, current: &[1, 10, 17, 25], // mutation\n   '),
    ('cx02_11_irlanda_do_norte_fase_3',
     'GroupCupProgress { knockout: 3, current: &[1, 5, 9, 13, 17],',
     'GroupCupProgress { knockout: 4, current: &[1, 5, 9, 13, 17], // mutation\n   '),
]
