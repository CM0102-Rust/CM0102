# Mutacoes (cx02_5), carreira ENG ate 2002-06-01 com o modo de textos. Nenhuma abre caixa.
# - sem o link da competicao na manchete do campeao da secao: o comparador de texto acusa (links diferentes).
MUTACOES = [
    ('cx02_5_sem_link_da_competicao',
     '        exe::text_add_link(text, (competition.addr() + 0x38) as *const u8, 0x7d0, competition.addr());',
     '        // mutation: no competition link'),
]
