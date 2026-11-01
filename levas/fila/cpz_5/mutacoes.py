# Mutacoes (cpz_5), na soma do resultado na linha da tatica (0x896290), que roda no fim de cada partida sem humano
# (o laco do motor, 0x6ba9a0). A escolha da formacao do computador (0x6e0230, julgar_formacoes) le de cada linha os
# jogos (+2, peso ate 3000) e a razao da postura (+0x10, +0x18 ou +0x20, ate 3): a vitoria valendo 1 ponto, os gols
# contra no lugar dos pro ou a razao da postura normal sem o ultimo fator q mudam a nota das formacoes, a formacao
# escolhida e as partidas seguintes (sorteios e resultados), que a carreira le. O contador fica depois das condicoes
# (jogo nao nulo, nenhum humano, clube nao nulo), colado na soma.
MUTACOES = [
    ('cpz5_vitoria_vale_1',
     '*m(0xe) = (*m(0xe)).wrapping_add(3); // cpz5: a vitória vale 3 pontos',
     '*m(0xe) = (*m(0xe)).wrapping_add(1); // mutacao: a vitoria vale 1 ponto',
     'cpz_5 linha da tatica'),
    ('cpz5_gols_contra_como_pro',
     '*m(0xa) = (*m(0xa)).wrapping_add(pro as i16); // cpz5: os gols pró',
     '*m(0xa) = (*m(0xa)).wrapping_add(contra as i16); // mutacao: soma os gols contra',
     'cpz_5 linha da tatica'),
    ('cpz5_razao_normal_sem_q',
     'core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: a razão da postura normal',
     'core::ptr::write_unaligned(f(0x18), r * g * q2); // mutacao: sem o ultimo fator q',
     'cpz_5 linha da tatica'),
]
