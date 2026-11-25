# Mutações (rx05_4). As três notícias só vão às caixas dos técnicos (contagem 0 na carreira ENG e no mundo até
# 2002-08-01) e o comparador é só a original: a mutação (o tipo da cláusula trocado) escapa da cadeia por não rodar
# (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_4_tipo_da_clausula',
     'exe::newsitem_set_slot(n, 6, kind as i8 as i32 as u32);',
     'exe::newsitem_set_slot(n, 6, (kind as i8 as i32 as u32) ^ 1); // mutation',
     'rx05_4 release clause news'),
]
