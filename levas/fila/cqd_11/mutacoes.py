# Mutacoes (cqd_11): as tres na 0x41a630 (a noticia da falta ao treino, 81 chamadas ate 2002-06-01). As duas primeiras
# rotinas rodam (1212 e 242 chamadas), mas na carreira automatica nenhuma pessoa chega a elas com o bit (ninguem sumiu e
# quem faltou ao treino nao ficou sem contrato): os contadores do bit nao rodaram, e mutacao nelas escaparia. A noticia
# so vai as caixas dos humanos, mas sorteia: a carreira pega pela contagem de sorteios. Sem o sorteio(3), um sorteio a
# menos por noticia; com o relator invertido, o clube da IA com tecnico passa a sortear o treinador (sorteio(2) e os
# sorteio(5)); sem a noticia, o sorteio(3) e o resto somem. Os contadores "cqd_11 noticia da falta" e "cqd_11 relator
# tecnico" mostram quantas vezes cada ramo roda. Podem escapar so se a noticia nao rodar ate a data.
MUTACOES = [
    ('cqd11_sem_sorteio', '    let r = cqd11_sortear(3); // cqd11: o sorteio da notícia',
     '    let r = 0; // mutacao: sem o sorteio da noticia',
     'cqd_11 noticia da falta'),
    ('cqd11_relator_invertido', '    if tecnico != 0 && humano(clube, 1, 1, 0, 0) == 0 { // cqd11: o técnico da IA relata',
     '    if tecnico != 0 && humano(clube, 1, 1, 0, 0) != 0 { // mutacao: o relator invertido',
     'cqd_11 relator tecnico'),
    ('cqd11_sem_noticia', '    if jogador == 0 || cqd11_d(jogador + 0x61) == 0 || clube == 0 || cqd11_d(clube + 0xcf) == 0 { // cqd11: sem jogador ou técnico',
     '    if true { // mutacao: sem a noticia',
     'cqd_11 noticia da falta'),
]
