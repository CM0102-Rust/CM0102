# Mutacoes (cqz_9): a Copa da Asia sem as honras da temporada (a rotina roda 2 vezes na carreira: o historico fica sem
# elas); as duas primeiras previsoes da Copa da Oceania trocadas (o sorteio de 2002 grava as previsoes nas estatisticas);
# a repescagem da Oceania sem esperar o fim das eliminatorias sul-americanas (a fase 2 nasce antes).
MUTACOES = [
    ('cqz9_asia_sem_honras',
     'let stage = read_usize(this.stages() + 8); // cqz9: the knockout is the third stage',
     'if this.addr() != 0 { return; } // mutation: no honours\n    let stage = read_usize(this.stages() + 8);',
     'cqz_9 asian cup honours'),
    ('cqz9_oceania_previsoes_trocadas',
     'const OCEANIA_NATIONS_PREDICTED_PLACES: [u32; 6] = [1, 2, 3, 3, 5, 5];',
     'const OCEANIA_NATIONS_PREDICTED_PLACES: [u32; 6] = [2, 1, 3, 3, 5, 5];',
     'cqz_9 oceania nations predictions'),
    ('cqz9_repescagem_sem_esperar',
     'if complete(south, 1, 1) == 0 { // cqz9: the play-off waits for the South American qualifying',
     'if complete(south, 1, 1) == 2 { // mutation: no wait',
     'cqz_9 oceania qualifying next stage'),
]
