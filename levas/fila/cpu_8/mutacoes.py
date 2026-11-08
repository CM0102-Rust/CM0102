# Mutacoes. Todo jogador do computador treina pelo programa 1 (geral) ou pelo 6 (goleiros) da ficha de treino
# (0x8b39f0): os niveis entram no progresso de cada categoria em cada atualizacao do treino (0x8b1750, na escalacao de
# cada partida), e os atributos treinados vao para o motor das partidas. O treino geral com 50 na primeira categoria, o
# dos goleiros com 25 na quinta e o programa 1 do computador com o tipo 2 (50 na primeira categoria, 10 nas outras)
# mudam o progresso de milhares de jogadores. A montagem roda uma vez no comeco de cada carreira nova.
MUTACOES = [
    ('cpu8_geral_mais_forte', 'set_schedule_levels(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral',
     'set_schedule_levels(this, [0x32, 0x19, 0x19, 0x19, 0]); // mutacao: 50 na primeira categoria',
     'cpu_8 geral'),
    ('cpu8_goleiros_mais_fraco', 'set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: o treino dos goleiros',
     'set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x19]); // mutacao: 25 na quinta categoria',
     'cpu_8 goleiros'),
    ('cpu8_computador_tipo_2', 'tipo(this + 0x9b, 1, 1); // cpu8: o programa geral do computador',
     'tipo(this + 0x9b, 1, 2); // mutacao: o programa 1 do computador com o tipo 2',
     'cpu_8 programas padrao'),
]
