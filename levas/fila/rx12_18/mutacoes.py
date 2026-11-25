# Mutacoes (rx12_18), pela carreira com a Belgica ate 2002-08-01. O texto nao volta a simulacao: acusado pelo
# comparador lado a lado (compare_text_writer), que conta como acusada. A lista: acusada pelo comparador (o plano), se
# alguem a pedir na janela.
# - o play-off de acesso sem o caso da segunda divisao ("win promotion" sempre);
# - a lista com a contagem de um grupo so (16).
MUTACOES = [
    ('rx12_18_sem_segunda',
     '            let against_second = a7 == read_u32(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION); // rx12_18: a play-off with a Second Division club',
     '            let against_second = false; // mutation',
     'rx12_18 belgian third play-off fate'),
    ('rx12_18_contagem',
     '    *n = 0x20; // rx12_18: the count of the two groups',
     '    *n = 0x10; // mutation',
     'rx12_18 belgian third clubs'),
]
