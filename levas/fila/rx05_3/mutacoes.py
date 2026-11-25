# Mutações (rx05_3). As três notícias só vão às caixas dos técnicos humanos (contagem 0 na carreira ENG e no mundo até
# 2002-08-01) e o comparador é só a original: a mutação (a quantidade de jogadores da troca sem permissão com um a
# mais) escapa da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_3_sem_permissao_mais_um',
     'exe::newsitem_set_slot(n, 7, count as i32 as u32);',
     'exe::newsitem_set_slot(n, 7, count as i32 as u32 + 1); // mutation',
     'rx05_3 exchange player needs a permit'),
]
