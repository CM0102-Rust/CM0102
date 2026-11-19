# Mutacoes (mwg_15) (textos: pegas pelo comparador lado a lado, nao pela carreira):
# - a eliminacao no grupo da Copa fica sem texto (a Copa de 2002, junho);
# - o premio escreve o corpo no lugar da manchete e vice-versa.
# add_shortlist fica sem mutacao propria: o comparador so roda o original (realoca) e o indice que ela devolve so e
# lido na votacao dos humanos.
MUTACOES = [
    ('mwg15_eliminado_sem_texto',
     "0xfe => -2, // mwg15: the group exit has its text",
     "0xfe => return, // mutation: no text for the group exit",
     'mwg_15 world cup group fate'),
    ('mwg15_corpo_trocado',
     "write(object, item, text, body) // mwg15: headline or body",
     "write(object, item, text, (body == 0) as u32) // mutation: headline and body swapped",
     'mwg_15 award news'),
]
