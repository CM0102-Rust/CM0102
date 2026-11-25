# Mutações (rx05_1). As três notícias só vão às caixas dos técnicos humanos e têm contagem 0 na carreira ENG e no
# mundo até 2002-08-01: a carreira não as acusa e o comparador é só a original. A mutação abaixo (a quantidade de
# jogadores da troca com um a mais) fica registrada para quando houver um humano numa negociação com troca; ela escapa
# da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_1_troca_mais_um',
     'exe::newsitem_set_slot(n, 0xd, count as i32 as u32);',
     'exe::newsitem_set_slot(n, 0xd, count as i32 as u32 + 1); // mutation',
     'rx05_1 exchange player in the negotiation news'),
]
