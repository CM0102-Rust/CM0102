# Mutações (rx05_2). As três notícias só vão às caixas dos técnicos humanos (contagem 0 na carreira ENG e no mundo até
# 2002-08-01) e o comparador é só a original: a mutação (o ano do fim do empréstimo + 1) escapa da cadeia por não rodar
# (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_2_ano_do_emprestimo',
     'exe::newsitem_set_slot(n, 9, terms.contract_expires_year() as u16 as u32);',
     'exe::newsitem_set_slot(n, 9, terms.contract_expires_year() as u16 as u32 + 1); // mutation',
     'rx05_2 loan accepted news'),
]
