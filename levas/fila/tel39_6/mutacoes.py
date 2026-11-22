# Mutacoes (tel39_6). O texto nao volta para a simulacao: acusadas pelo comparador do proprio metodo (lado a lado),
# como as de texto aceitas em 12/11/2026.
# - o texto acrescentado vai antes do antigo (toda noticia de varias partes);
# - a mesma frase ja ligada a outro objeto responde 1 (noticia que cita o mesmo nome duas vezes com vinculos
#   diferentes).
MUTACOES = [
    ('tel39_6_ordem',
     'format(p, COMP_TEXT_JOIN, old, source);',
     'format(p, COMP_TEXT_JOIN, source, old);',
     'tel39_6 append to the string'),
    ('tel39_6_mesma_frase',
     'return (read_u32(e + TEXT_LINK_KIND) == kind && read_u32(e + TEXT_LINK_DATA) == data) as u32;',
     'return (read_u32(e + TEXT_LINK_KIND) == kind || read_u32(e + TEXT_LINK_DATA) == data) as u32;',
     'tel39_6 link already there'),
]
