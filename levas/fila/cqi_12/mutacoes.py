# Mutacoes (cqi_12), nas regras dos grupos das copas italianas (0x63f350 e 0x63d6e0, uma chamada cada na montagem das
# copas; os grupos jogam em agosto de 2001): a vitoria valendo 2 muda a tabela e quem passa; a categoria 6 vai para cada
# jogo (+0x3a da partida: publico, renda, escalacao); dois turnos na Serie C dobram os jogos do grupo. A carreira pega
# pelos jogos (e pelos sorteios das partidas) no fim de agosto.
MUTACOES = [
    ('cqi12_vitoria_vale_dois_na_copa_da_italia',
     'g.vitoria = 3; // cqi12: a vitória nos grupos da Copa da Itália',
     'g.vitoria = 2; // mutacao: a vitoria vale 2',
     'cqi_12 italy cup rules'),
    ('cqi12_categoria_6_na_copa_da_italia',
     'g.categoria = 4; // cqi12: a categoria dos jogos da Copa da Itália',
     'g.categoria = 6; // mutacao: os jogos como fase final',
     'cqi_12 italy cup rules'),
    ('cqi12_dois_turnos_na_serie_c',
     'let turnos = 1i16; // cqi12: o turno único dos grupos da Série C',
     'let turnos = 2i16; // mutacao: dois turnos',
     'cqi_12 serie c cup rules'),
]
