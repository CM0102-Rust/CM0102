# Mutacoes (cx14_6), com a Turquia ate 2002-08-01:
# - os clubes de cada grupo da primeira fase de tras para frente na lista (a tabela de jogos do grupo sai da ordem da
#   lista: outros jogos em cada rodada; o comparador confere os clubes como conjunto e nao a acusa);
# - o grupo de acesso com 3 que sobem (muda quem sobe na virada do verao de 2002);
# - os grupos de classificacao sem os pontos da primeira fase (muda quem cai e quem vai ao mata-mata; acusada tambem
#   pelo comparador, que confere o registro levado).
MUTACOES = [
    ('cx14_6_grupo_invertido',
     'write_usize(list + 4 * k, club); // cx14_6: the k-th club of the group',
     'write_usize(list + 4 * (9 - k), club); // mutation: backwards',
     'cx14_6 category b group stage'),
    ('cx14_6_acesso_tres_sobem',
     '&[2, 3, 0, 0], 4, 0x14, 3); // cx14_6: 2 up, 3 to the play-offs',
     '&[3, 3, 0, 0], 4, 0x14, 3); // mutation: 3 up',
     'cx14_6 category b promotion group'),
    ('cx14_6_classificacao_sem_pontos',
     'carry_first_phase_record(from, row); // cx14_6: the first phase record carried over',
     'let _ = (from, row); // mutation: nothing carried over',
     'cx14_6 category b classification group'),
]
