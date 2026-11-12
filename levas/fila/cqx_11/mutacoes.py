# Mutacoes (cqx_11), no jogo da Intertoto (401 chamadas ate 2002-06-01; as rodadas de julho e agosto de 2001, com
# clubes da Inglaterra, Espanha, Italia e Franca): a vitoria do mandante vale 1 ponto de coeficiente em vez de 2 (os
# coeficientes das nacoes mudam: o comparador pega na confirmacao ate 2001-08-20); sem a base da copa (o jogo nao e
# processado: quem passa de fase muda). Os registros do LDV e da Oceania sao passageiros (sem mutacao).
MUTACOES = [
    ('cqx11_vitoria_vale_1',
     'add_uefa_coefficient(g.home_club().nation().addr(), 2); // cqx11: the win\'s 2 points',
     'add_uefa_coefficient(g.home_club().nation().addr(), 1); // mutation: 1 point',
     'cqx_11 intertoto coefficient'),
    ('cqx11_sem_a_base_da_copa',
     'exe::cup_process_fixture(this, game, arg); // cqx11: the cup base plays the match out',
     '// mutation: without the cup base',
     'cqx_11 intertoto coefficient'),
]
