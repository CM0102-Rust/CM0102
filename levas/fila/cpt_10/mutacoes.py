# Mutacoes, todas na semana do treino (todo dia pelo laco do dia; o contador roda uma vez por semana). A
# update_player_training (0x8b1750, na escalacao de cada partida) poe em dia as semanas que faltam de cada jogador
# (+0x688d menos a semana dele): com a virada no dia seguinte, os jogadores que jogam no dia da virada treinam uma
# semana depois e os sorteios do progresso caem em outros momentos; com duas semanas por virada, o dobro do treino; com
# a semana de 6 dias, a virada em outros dias e mais semanas no ano. Os atributos mudam e o motor das partidas os le.
MUTACOES = [
    ('cpt10_virada_um_dia_depois', 'if resto == 0 { // cpt10: o sétimo dia',
     'if resto == 1 { // mutacao: a virada um dia depois',
     'cpt_10 semana'),
    ('cpt10_duas_semanas', '*s = (*s).wrapping_add(1); // cpt10: mais uma semana',
     '*s = (*s).wrapping_add(2); // mutacao: duas semanas',
     'cpt_10 semana'),
    ('cpt10_semana_de_6_dias', 'let resto = v % 7; // cpt10: a semana de 7 dias',
     'let resto = v % 6; // mutacao: a semana de 6 dias',
     'cpt_10 semana'),
]
