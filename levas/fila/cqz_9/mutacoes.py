# Mutacoes (cqz_9): a Copa da Asia sem as honras da temporada (a rotina roda 2 vezes na carreira: o historico fica sem
# elas); as duas primeiras previsoes da Copa da Oceania trocadas (o sorteio de 2002 grava as previsoes nas estatisticas);
# a repescagem da Oceania sem esperar o fim das eliminatorias sul-americanas (a fase 2 nasce antes).
MUTACOES = [
    ('cqz9_asia_sem_honras',
     'let stage = read_usize(this.stages() + 8); // cqz9: the knockout is the third stage',
     'if this.addr() != 0 { return; } // mutation: no honours\n    let stage = read_usize(this.stages() + 8);',
     'cqz_9 asian cup honours'),
    # 14/11/2026: a mutacao antiga mexia na constante que a copia E o comparador usam como referencia, logo nunca
    # discriminava; agora muda o comportamento da copia (a posicao vai para a selecao seguinte)
    ('cqz9_previsao_do_vizinho',
     'read_usize(v + k * 4), p); // cqz9: the place by reputation',
     'read_usize(v + ((k + 1) % 6) * 4), p); // mutation: the place goes to the next nation',
     'cqz_9 oceania nations predictions'),
    ('cqz9_repescagem_sem_esperar',
     'if complete(south, 1, 1) == 0 { // cqz9: the play-off waits for the South American qualifying',
     'if complete(south, 1, 1) == 2 { // mutation: no wait',
     'cqz_9 oceania qualifying next stage'),
]
